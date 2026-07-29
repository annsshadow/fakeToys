package config

// Config 顶层配置
type Config struct {
	App    AppConfig
	Batches []BatchConfig
}

// AppConfig 应用配置
type AppConfig struct {
	DingTalkAppKey       string `mapstructure:"dingtalk_app_key"`
	DingTalkAppSecret    string `mapstructure:"dingtalk_app_secret"`
	GroupConversationID  string `mapstructure:"group_conversation_id"`
}

// BatchConfig 批次配置
type BatchConfig struct {
	Message     string   `mapstructure:"message"`
	AccountIDs  []string `mapstructure:"account_ids"`
}
