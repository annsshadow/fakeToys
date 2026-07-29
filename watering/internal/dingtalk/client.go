package dingtalk

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"sync"
	"time"

	"go.uber.org/zap"
)

const (
	// 钉钉开放API地址
	getTokenURL  = "https://api.dingtalk.com/v1.0/oauth2/accessToken"
	sendMsgURL   = "https://api.dingtalk.com/v1.0/im/groupMessages/send"
)

// Client 钉钉客户端
type Client struct {
	appKey     string
	appSecret  string
	httpClient *http.Client
	logger     *zap.SugaredLogger

	tokenMu sync.RWMutex
	token   string
	expires time.Time
}

// NewClient 创建客户端
func NewClient(appKey, appSecret string, logger *zap.SugaredLogger) *Client {
	return &Client{
		appKey:     appKey,
		appSecret:  appSecret,
		httpClient: &http.Client{Timeout: 10 * time.Second},
		logger:     logger,
	}
}

// tokenResponse 获取access_token响应
type tokenResponse struct {
	AccessToken string `json:"accessToken"`
	ExpireIn    int64  `json:"expireIn"`
}

// sendGroupMessageRequest 发送群消息请求
type sendGroupMessageRequest struct {
	ConversationID string       `json:"conversationId"`
	MsgKey         string       `json:"msgKey"`
	MsgParam       string       `json:"msgParam"`
	UserIDs        []string     `json:"userIds"`
}

// sendGroupMessageResponse 发送群消息响应
type sendGroupMessageResponse struct {
	InvalidUserIDs   []string `json:"invalidUserIds"`
	ForbiddenUserIDs []string `json:"forbiddenUserIds"`
}

// SendResult 发送结果
type SendResult struct {
	FailedUserIDs  []string
	ErrorMessages map[string]string
}

// GetAccessToken 公开方法：获取access_token（暴露给main做连通性验证）
func (c *Client) GetAccessToken(ctx context.Context) (string, error) {
	return c.getAccessToken(ctx)
}
func (c *Client) getAccessToken(ctx context.Context) (string, error) {
	c.tokenMu.RLock()
	if c.token != "" && time.Now().Before(c.expires) {
		token := c.token
		c.tokenMu.RUnlock()
		return token, nil
	}
	c.tokenMu.RUnlock()

	c.tokenMu.Lock()
	defer c.tokenMu.Unlock()

	// 双重检查
	if c.token != "" && time.Now().Before(c.expires) {
		return c.token, nil
	}

	reqBody, _ := json.Marshal(map[string]string{
		"appKey":    c.appKey,
		"appSecret": c.appSecret,
	})

	req, err := http.NewRequestWithContext(ctx, "POST", getTokenURL, bytes.NewReader(reqBody))
	if err != nil {
		return "", err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("获取access_token失败: %s", string(body))
	}

	var tr tokenResponse
	if err := json.Unmarshal(body, &tr); err != nil {
		return "", err
	}

	c.token = tr.AccessToken
	c.expires = time.Now().Add(time.Duration(tr.ExpireIn-600) * time.Second) // 提前10分钟过期

	c.logger.Infow("获取access_token成功", "expires_in", tr.ExpireIn)
	return c.token, nil
}

// SendGroupMessage 发送群消息
func (c *Client) SendGroupMessage(ctx context.Context, conversationID, message string, userIDs []string) (*SendResult, error) {
	token, err := c.getAccessToken(ctx)
	if err != nil {
		return nil, err
	}

	// 注意：这里使用text类型消息，具体msgKey/msgParam格式需根据实际API调整
	msgParam, _ := json.Marshal(map[string]string{"content": message})

	reqBody, _ := json.Marshal(sendGroupMessageRequest{
		ConversationID: conversationID,
		MsgKey:         "sampleText",
		MsgParam:       string(msgParam),
		UserIDs:        userIDs,
	})

	req, err := http.NewRequestWithContext(ctx, "POST", sendMsgURL, bytes.NewReader(reqBody))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("x-acs-dingtalk-access-token", token)

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("发送群消息失败: %s", string(body))
	}

	var sgResp sendGroupMessageResponse
	if err := json.Unmarshal(body, &sgResp); err != nil {
		return nil, err
	}

	failed := append(sgResp.InvalidUserIDs, sgResp.ForbiddenUserIDs...)
	errMap := make(map[string]string)
	for _, id := range failed {
		errMap[id] = "send_failed"
	}

	c.logger.Infow("发送群消息完成",
		"success", len(userIDs)-len(failed),
		"failed", len(failed),
	)

	return &SendResult{
		FailedUserIDs:  failed,
		ErrorMessages:  errMap,
	}, nil
}
