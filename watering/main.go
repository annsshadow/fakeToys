package main

import (
	"context"
	"fmt"
	"time"

	"github.com/spf13/viper"
	"go.uber.org/zap"

	"watering/internal/config"
	"watering/internal/dingtalk"
)

const (
	configFile = "config.yaml"
)

func main() {
	// 初始化日志
	logger, _ := zap.NewProduction()
	defer logger.Sync()
	log := logger.Sugar()

	// 加载配置
	v := viper.New()
	v.SetConfigFile(configFile)
	v.SetConfigType("yaml")
	v.AutomaticEnv()

	if err := v.ReadInConfig(); err != nil {
		log.Fatalf("无法读取配置文件 %s: %v", configFile, err)
	}

	var cfg config.Config
	if err := v.Unmarshal(&cfg); err != nil {
		log.Fatalf("配置解析失败: %v", err)
	}

	// 校验必填项
	if cfg.App.DingTalkAppKey == "" || cfg.App.DingTalkAppSecret == "" {
		log.Fatal("请设置 DINGTALK_APP_KEY 和 DINGTALK_APP_SECRET 环境变量，或修改 config.yaml")
	}

	log.Infow("配置加载成功",
		"app_key", cfg.App.DingTalkAppKey,
		"group_id", cfg.App.GroupConversationID,
		"batch_count", len(cfg.Batches),
	)

	// 初始化钉钉客户端
	client := dingtalk.NewClient(cfg.App.DingTalkAppKey, cfg.App.DingTalkAppSecret, log)

	// 测试：获取 access_token
	ctx, cancel := context.WithTimeout(context.Background(), 15*time.Second)
	defer cancel()

	token, err := client.GetAccessToken(ctx)
	if err != nil {
		log.Fatalf("获取 access_token 失败: %v", err)
	}
	fmt.Printf("✓ 获取 access_token 成功 (长度: %d)\n", len(token))

	// 测试：发送群消息（发到第一个 batch，取第一个账号测试）
	if len(cfg.Batches) > 0 && len(cfg.Batches[0].AccountIDs) > 0 {
		batch := cfg.Batches[0]
		testUserID := batch.AccountIDs[0]
		result, err := client.SendGroupMessage(ctx, cfg.App.GroupConversationID, batch.Message, []string{testUserID})
		if err != nil {
			log.Fatalf("发送消息失败: %v", err)
		}
		log.Infow("发送完成",
			"batch", batch.Message,
			"user", testUserID,
			"failed_count", len(result.FailedUserIDs),
			"errors", result.ErrorMessages,
		)
		fmt.Printf("✓ 发送消息完成 (批次=%s, 测试账号=%s, 失败数=%d)\n", batch.Message, testUserID, len(result.FailedUserIDs))
	} else {
		fmt.Println("⚠ 没有配置测试账号，跳过发送测试")
	}
}
