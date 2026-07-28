package main

import (
	"encoding/csv"
	"io"
	"net/http"
	"time"

	"github.com/huaweicloud/huaweicloud-sdk-go-obs/obs"
	"go.uber.org/zap"
)

var logger *zap.Logger

func init() {
	// 初始化zap日志库
	var err error
	logger, err = zap.NewProduction()
	if err != nil {
		panic(err)
	}
	defer logger.Sync()
}

func main() {
	obsClient := initOBSClient()

	http.HandleFunc("/readcsv", func(w http.ResponseWriter, r *http.Request) {
		startTime := time.Now()
		handleOBSRequest(w, r, obsClient)
		logger.Info("Request processed",
			zap.String("method", r.Method),
			zap.String("path", r.URL.Path),
			zap.Duration("duration", time.Since(startTime)),
		)
	})

	logger.Info("Server is starting...")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		logger.Fatal("Server failed to start", zap.Error(err))
	}
}

func initOBSClient() *obs.ObsClient {
	ak := os.getenv("OBS_AK", "<YOUR_ACCESS_KEY_ID>")
	sk := os.getenv("OBS_SK", "<YOUR_SECRET_ACCESS_KEY>")
	endpoint := "YOUR_ENDPOINT"

	obsClient, err := obs.New(ak, sk, endpoint)
	if err != nil {
		logger.Fatal("Failed to initialize OBS client", zap.Error(err))
	}

	return obsClient
}

func handleOBSRequest(w http.ResponseWriter, r *http.Request, obsClient *obs.ObsClient) {
	bucketName := "YOUR_BUCKET_NAME"
	objectKey := "example.csv"

	inputStream, err := obsClient.GetObject(&obs.GetObjectInput{Bucket: bucketName, Key: objectKey})
	if err != nil {
		logger.Error("Failed to get object from OBS", zap.Error(err))
		http.Error(w, "Failed to get object from OBS", http.StatusInternalServerError)
		return
	}
	defer inputStream.Body.Close()

	reader := csv.NewReader(inputStream.Body)

	for {
		record, err := reader.Read()
		if err == io.EOF {
			break
		}
		if err != nil {
			logger.Error("Failed to read CSV", zap.Error(err))
			http.Error(w, "Failed to read CSV", http.StatusInternalServerError)
			return
		}

		_, writeErr := w.Write([]byte(string.Join(record, ",") + "\n"))
		if writeErr != nil {
			logger.Error("Failed to write response", zap.Error(writeErr))
			// 如果出错，尝试通知客户端出错了
			http.Error(w, "Failed to write response", http.StatusInternalServerError)
			return
		}
	}
}