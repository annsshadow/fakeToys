package main

import (
	"bufio"
	"database/sql"
	"encoding/csv"
	"fmt"
	"io"
	"log"
	"os"
	"strings"

	"github.com/huaweicloud/huaweicloud-sdk-go-obs/obs"
	_ "github.com/mattn/go-sqlite3"
)

func main() {
	// 设置日志文件
	logFile, err := os.OpenFile("process.log", os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0666)
	if err != nil {
		log.Fatalf("Error opening log file: %v", err)
	}
	defer logFile.Close()
	log.SetOutput(logFile)

	// OBS配置
	ak := os.Getenv("OBS_AK")
	if ak == "" {
		ak = "<YOUR_ACCESS_KEY_ID>"
	}
	sk := os.Getenv("OBS_SK")
	if sk == "" {
		sk = "<YOUR_SECRET_ACCESS_KEY>"
	}
	endpoint := "your-obs-endpoint"
	bucketName := "your-bucket-name"
	objectKey := "your-object-key"

	// 初始化OBS客户端
	obsClient, err := obs.New(ak, sk, endpoint)
	if err != nil {
		log.Fatalf("Failed to create OBS client: %v", err)
	}

	// 下载CSV文件并流式读取
	input := &obs.GetObjectInput{
		GetObjectMetadataInput: obs.GetObjectMetadataInput{Bucket: bucketName, Key: objectKey},
	}
	output, err := obsClient.GetObject(input)
	if err != nil {
		log.Fatalf("Failed to get object: %v", err)
	}
	defer output.Body.Close()

	// 创建SQLite内存数据库
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		log.Fatalf("Failed to open SQLite database: %v", err)
	}
	defer db.Close()

	// 读取CSV头部，并创建对应表
	reader := csv.NewReader(bufio.NewReader(output.Body))
	headers, err := readCSVHeader(reader)
	if err != nil {
		log.Fatalf("Failed to read CSV header: %v", err)
	}

	if err := createTable(db, headers); err != nil {
		log.Fatalf("Failed to create table: %v", err)
	}

	// 插入CSV行到表中
	if err := insertCSVData(reader, db, headers); err != nil {
		log.Fatalf("Failed to insert CSV data: %v", err)
	}

	// 数据操作完成，此处可以添加查询逻辑
	// ...
}

// readCSVHeader reads the first line of the CSV to get the headers
func readCSVHeader(reader *csv.Reader) ([]string, error) {
	headers, err := reader.Read()
	if err != nil {
		return nil, err
	}
	return headers, nil
}

// createTable creates a SQLite table to match the CSV structure
func createTable(db *sql.DB, headers []string) error {
	columnDefs := make([]string, len(headers))
	for i, field := range headers {
		columnDefs[i] = fmt.Sprintf("%q TEXT", field)
	}
	query := fmt.Sprintf("CREATE TABLE data (%s);", strings.Join(columnDefs, ", "))
	_, err := db.Exec(query)
	if err != nil {
		return err
	}
	return nil
}

// insertCSVData reads the CSV data and inserts it into the SQLite database
func insertCSVData(reader *csv.Reader, db *sql.DB, headers []string) error {
	tx, err := db.Begin()
	if err != nil {
		return err
	}
	defer tx.Rollback() // Make sure to rollback on error.

	stmt, err := tx.Prepare(fmt.Sprintf("INSERT INTO data (%s) VALUES (?%s)",
		strings.Join(headers, ","), strings.Repeat(",?", len(headers)-1)))
	if err != nil {
		return err
	}
	defer stmt.Close()

	count := 0
	for {
		record, err := reader.Read()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}

		if _, err := insertRecord(stmt, record); err != nil {
			log.Printf("Insert failed for record %d: %v", count, err)
			continue // Skip the bad record or return error depending on your needs
		}
		count++
	}

	if err := tx.Commit(); err != nil {
		return err
	}

	log.Printf("Successfully inserted %d records into the database", count)
	return nil
}

// insertRecord takes a prepared statement and a CSV record and executes the insert operation
func insertRecord(stmt *sql.Stmt, record []string) (sql.Result, error) {
	recordInterface := make([]interface{}, len(record))
	for i := range record {
		recordInterface[i] = record[i]
	}
	result, err := stmt.Exec(recordInterface...)
	if err != nil {
		return nil, err
	}
	return result, nil
}