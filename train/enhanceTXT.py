import os
import nlpaug.augmenter.char as nac
import nlpaug.augmenter.word as naw
import nlpaug.augmenter.sentence as nas
import nlpaug.flow as nafc
from nlpaug.util import Action
from nlpcda import Similarword
from nlpcda import Homophone
# import torch
import json
import re
from datetime import datetime
import requests
import threading
import time
from queue import Queue
from nlpcda import Simbert
import logging

######################################################################
# 同义替换规则或模板
synonym_rules = [
    (r'(\b如何\b)', '怎么样'),
    (r'(\b请问\b)?(\b流程\b)', '步骤是什么'),
    # 可以添加更多规则...
]

os.environ["MODEL_DIR"] = './'

API_KEY = os.getenv("BAIDU_API_KEY", "YOUR_BAIDU_API_KEY")
SECRET_KEY = os.getenv("BAIDU_SECRET_KEY", "YOUR_BAIDU_SECRET_KEY")

# 配置日志记录
logging.basicConfig(
    filename='app.log',      # 日志文件名
    filemode='a',            # 文件模式，'a' 为追加模式，'w' 为覆盖模式
    format='%(asctime)s - %(levelname)s - %(message)s',  # 日志格式
    level=logging.DEBUG       # 日志级别
)

# 创建日志记录器
logger = logging.getLogger()
logger.setLevel(logging.DEBUG)

# 清除已有的处理器，以避免重复记录
if logger.hasHandlers():
    logger.handlers.clear()

# 创建文件处理器
file_handler = logging.FileHandler('app.log')
file_handler.setLevel(logging.DEBUG)

# 创建控制台处理器
console_handler = logging.StreamHandler()
console_handler.setLevel(logging.ERROR)  # 只显示错误及以上级别的日志

# 设置格式
formatter = logging.Formatter('%(asctime)s - %(levelname)s - %(message)s')
file_handler.setFormatter(formatter)
console_handler.setFormatter(formatter)

# 添加处理器到日志记录器
logger.addHandler(file_handler)
logger.addHandler(console_handler)
######################################################################

def apply_synonym_rules(text):
    for pattern, replacement in synonym_rules:
        text = re.sub(pattern, replacement, text)
    return text


# print debug info
# print(torch.__version__)
# print('[debug] model dir: ' + os.environ.get("MODEL_DIR"))
def p(t1: str, t2: list):
    debug("Original:" + t1)
    debug("Augmented Text:" + t2[0])


def debug(s: str):
    logging.debug(f"[DEBUG] {datetime.now().strftime('%Y-%m-%d %H:%M:%S')} {s}")


def get_last_line():
    # 逐行读取文件
    last_line = None
    with open('./train_data_cached.json', 'r', encoding='utf-8') as file:
        for line in file:
            last_line = line.strip()  # 更新最后一行

    if last_line:
        logging.info("最后一行内容:", last_line)
        with open('./last_line.json', 'w', encoding='utf-8') as file:
            file.write(last_line + '\n')
            file.flush()
    else:
        logging.error("文件为空。")
######################################################################
class MyA:
    def __init__(self):
        # self.wea1 = naw.WordEmbsAug(model_type='word2vec',
        #                             model_path=os.environ.get("MODEL_DIR") + 'GoogleNews-vectors-negative300.bin')
        # self.wea2 = naw.WordEmbsAug(model_type='glove', model_path=os.environ.get("MODEL_DIR") + 'glove.6B.300d.txt')
        # self.wea3 = naw.WordEmbsAug(model_type='fasttext',
        #                             model_path=os.environ.get("MODEL_DIR") + 'wiki-news-300d-1M.vec')
        # debug("init WordEmbsAug success")
        #
        # self.cwea1 = naw.ContextualWordEmbsAug(model_path='bert-base-uncased', action="substitute")
        # self.cwea2 = naw.ContextualWordEmbsAug(model_path='distilbert-base-uncased', action="substitute")
        # self.cwea3 = naw.ContextualWordEmbsAug(model_path='roberta-base', action="substitute")
        # debug("init ContextualWordEmbsAug success")
        #
        # self.sa1 = naw.SynonymAug(aug_src='ppdb', model_path=os.environ.get("MODEL_DIR") + 'ppdb-2.0-tldr')
        # debug("init SynonymAug success")
        #
        # self.bta1 = naw.BackTranslationAug(from_model_name='Helsinki-NLP/opus-mt-zh-en',
        #                                    to_model_name='Helsinki-NLP/opus-mt-en-zh')
        # debug("init BackTranslationAug success")
        #
        # self.cwesa1 = nas.ContextualWordEmbsForSentenceAug(model_path='xlnet-base-cased')
        # self.cwesa2 = nas.ContextualWordEmbsForSentenceAug(model_path='gpt2')
        # self.cwesa3 = nas.ContextualWordEmbsForSentenceAug(model_path='distilgpt2')
        # debug("init ContextualWordEmbsForSentenceAug success")

        # config = {
        #     'model_path': os.environ.get("MODEL_DIR") + 'chinese_simbert_L-12_H-768_A-12',
        #     'CUDA_VISIBLE_DEVICES': '0,1',
        #     'max_len': 32,
        #     'seed': 1
        # }
        # self.sb1 = Simbert(config=config)

        self.sw1 = Similarword(create_num=9, change_rate=0.1)
        debug("init Similarword success")
        self.sh1 = Homophone(create_num=9, change_rate=0.1)
        debug("init Homophone success")

        self.processed_data = []

    def process_json_data(self, input_file_path: str, output_file_path: str):
        i: int = 0

        # load original json file
        with open(input_file_path, 'r', encoding='utf-8') as file:
            debug(f"open {input_file_path} success")
            data = json.load(file)
            with open(output_file_path, 'w', encoding='utf-8') as out_file:
                debug(f"open {output_file_path} success")
                for item in data:
                    new_item = {
                        'prompt': item['instruction'],
                        'response': item['output']
                    }
                # 每行是一个包含单个对象的数组
                out_file.write(json.dumps([new_item], ensure_ascii=False) + '\n')

    # for item in data:
    #     # 复制当前项
    #     new_item = item.copy()
    #     # 将原项都添加到结果列表中
    #     self.processed_data.append(new_item)
    #     debug(f"original item: {i} success")
    #
    #     # 应用同义词规则改写instruction字段
    #     nlist = self.sw1.replace(item['instruction'])
    #     for it in nlist:
    #         # 创建新的字典，包含改写后的instruction作为问题，以及原始的output
    #         new_item_question = {
    #             'instruction': it,
    #             'input': '',
    #             'output': new_item['output']
    #         }
    #         # 将新项添加到结果列表中
    #         self.processed_data.append(new_item_question)
    #     debug(f"append similar item: {i} success")
    #
    #     # 应用近义词规则改写instruction字段
    #     nlist = self.sh1.replace(item['instruction'])
    #     for it in nlist:
    #         # 创建新的字典，包含改写后的instruction作为问题，以及原始的output
    #         new_item_question = {
    #             'instruction': it,
    #             'input': '',
    #             'output': new_item['output']
    #         }
    #         # 将新项添加到结果列表中
    #         self.processed_data.append(new_item_question)
    #     debug(f"append synonym item: {i} success")
    #
    #     i += 1
    #
    # with open(output_file_path, 'w', encoding='utf-8') as file:
    #     debug(f"open {output_file_path} success")
    #     json.dump(self.processed_data, file, ensure_ascii=False, indent=4)


# def wea(self, text: str):
#     p(text, self.wea1.augment(text))
#     p(text, self.wea2.augment(text))
#     p(text, self.wea3.augment(text))
#
# def cwea(self, text: str):
#     p(text, self.cwea1.augment(text))
#     p(text, self.cwea2.augment(text))
#     p(text, self.cwea3.augment(text))
#
# def sa(self, text: str):
#     p(text, self.sa1.augment(text))
#
# def bta(self, text: str):
#     p(text, self.bta1.augment(text))
#
# def cwesa(self, text: str):
#     p(text, self.cwesa1.augment(text))
#     p(text, self.cwesa2.augment(text))
#     p(text, self.cwesa3.augment(text))

# def sw(self, text: str):
#     debug(text)
#     print(self.sw1.replace(text))

def get_access_token():
    """
    使用 AK，SK 生成鉴权签名（Access Token）
    :return: access_token，或是None(如果错误)
    """
    url = "https://aip.baidubce.com/oauth/2.0/token"
    params = {"grant_type": "client_credentials", "client_id": API_KEY, "client_secret": SECRET_KEY}
    return str(requests.post(url, params=params).json().get("access_token"))


def extract_content(s):
    start = s.find('[')
    end = s.find(']') + 1
    if start > 0 and end > start:
        return s[start:end]
    return None


class MyBD:
    def __init__(self):
        # ernie-lite-8k free now
        # self.pre_url = "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/ernie-lite-8k?access_token="
        # ERNIE-3.5-8K  limited free token
        self.pre_url = "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions?access_token="

        self.processed_data = []

    def process_json_data(self, input_file_path: str, output_file_path: str, cache_file_path: str, fail_file_path: str):
        i: int = 0
        # load original json file
        with (open(input_file_path, 'r', encoding='utf-8') as file,
              open(output_file_path, 'w', encoding='utf-8') as out_file,
              open(cache_file_path, 'w', encoding='utf-8') as cache,
              open(fail_file_path, 'w', encoding='utf-8') as failed):
            debug(f"open {input_file_path} success")
            data = json.load(file)
            debug(f"open {output_file_path} success")
            debug(f"open {cache_file_path} success")
            for item in data:
                debug(f" start to deal with {i}, instruction: {item['instruction']}")
                jstr = json.dumps([item], ensure_ascii=False)
                msg = "作为AI领域数据增强专家，你的任务是把原JSON对象中的instruction字段内容在保持相同含义的情况下，扩充为5种不同的提问方式，新JSON对象只需包含instruction字段，并将新JSON对象追加到JSON数组里面，注意最终结果只需要JSON数组，不需要任何说明文字，原JSON对象如下：" + jstr
                debug(jstr)
                response = self.ask(msg)
                debug(response.text)
                rsp_json = json.loads(response.text)

                if not rsp_json['is_truncated']:
                    for line in json.loads(extract_content(rsp_json['result'])):
                        line["input"] = ""
                        line["output"] = item["output"]
                        self.processed_data.append(line)
                    cpd = self.processed_data.copy()
                    cache.truncate()
                    cache.write(json.dumps(cpd, ensure_ascii=False) + '\n')
                    cache.flush()
                    debug(f"cache {i} done")
                    i += 1
                else:
                    failed.write(json.dumps(jstr, ensure_ascii=False) + '\n')
                    failed.flush()
                    debug(f"failed {i} done")
                    break
            # write the final output
            out_file.write(json.dumps(self.processed_data, ensure_ascii=False) + '\n')
            out_file.flush()
            debug(f"output {i} done")

    def ask(self, msg: str):
        url = self.pre_url + get_access_token()

        payload = json.dumps({
            "temperature": 0.99,
            "top_p": 0.95,
            "penalty_score": 1,
            "max_output_tokens": 2048,
            "disable_search": False,
            "enable_citation": False,
            "response_format": "text",
            "messages": [
                {"role": "user", "content": msg}
            ]
        })
        headers = {
            'Content-Type': 'application/json'
        }

        return requests.request("POST", url, headers=headers, data=payload)


# 定义工作线程的目标函数
def worker(queue):
    url = ("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions?access_token="
           + get_access_token())
    headers = {
        'Content-Type': 'application/json'
    }
    processed_data = []

    out_fpath = f'./train_data_processed_{threading.current_thread().name}.json'
    cache_fpath = f'./train_data_cached_{threading.current_thread().name}.json'
    fail_fpath = f'./train_data_failed_{threading.current_thread().name}.json'
    with (open(out_fpath, 'w', encoding='utf-8') as out_file,
          open(cache_fpath, 'w', encoding='utf-8') as cache,
          open(fail_fpath, 'w', encoding='utf-8') as failed):
        debug(f"open {out_fpath} success by {threading.current_thread().name}")
        debug(f"open {cache_fpath} success by {threading.current_thread().name}")
        debug(f"open {fail_fpath} success by {threading.current_thread().name}")
        while not queue.empty():
            item, idx = queue.get()
            debug(f'Index: {idx} Processing item: {item} by {threading.current_thread().name}' + '\n')

            msg = "作为AI领域数据增强专家，你的任务是把原JSON对象中的instruction字段内容在保持相同含义的情况下，扩充为5种不同的提问方式，新JSON对象只需包含instruction字段，并将新JSON对象追加到JSON数组里面，注意最终结果只需要JSON数组，不需要任何说明文字，原JSON对象如下：" + item
            payload = json.dumps({
                "temperature": 0.99,
                "top_p": 0.95,
                "penalty_score": 1,
                "max_output_tokens": 2048,
                "disable_search": False,
                "enable_citation": False,
                "response_format": "text",
                "messages": [
                    {"role": "user", "content": msg}
                ]
            })

            response = requests.request("POST", url, headers=headers, data=payload)
            debug(response.text)
            rsp_json = json.loads(response.text)

            if not rsp_json['is_truncated']:
                jitem = json.loads(item)
                for line in json.loads(extract_content(rsp_json['result'])):
                    line["input"] = ""
                    line["output"] = jitem[0]["output"]
                    processed_data.append(line)
                cpd = processed_data.copy()
                cache.write(json.dumps(cpd, ensure_ascii=False) + '\n')
                cache.flush()
                debug(f"{threading.current_thread().name} cache {idx} done")
            else:
                failed.write(json.dumps(item, ensure_ascii=False) + '\n')
                failed.flush()
                debug(f"{threading.current_thread().name} failed {idx} done")
            queue.task_done()

        # write the final output
        out_file.write(json.dumps(processed_data, ensure_ascii=False) + '\n')
        out_file.flush()
        debug(f"{threading.current_thread().name} output {idx} done")

def main():
    ######################################################################
    in_fpath = './train_data.json'
    out_fpath = './train_data_processed.json'
    cache_fpath = './train_data_cached.json'
    fail_fpath = './train_data_failed.json'
    debug('-----------------------------------------------------------')
    # my = MyA()
    # txt = "请详述在日常生活中如何预防高空抛物和防范高空坠物？"
    # # synonyms = my.sb1.replace(sent=txt, create_num=9)
    # # 调用函数处理数据
    # my.process_json_data(in_fpath, out_fpath)

    # my.wea(txt)
    # my.cwea(txt)
    # my.sa(txt)
    # my.bta(txt)
    # my.cwesa(txt)
    # my.sw(txt)
    debug('-----------------------------------------------------------')
    txt = "The quick brown fox jumps over the lazy dog ."
    # my.wea(txt)
    # my.cwea(txt)
    # my.sa(txt)
    # my.bta(txt)
    # my.cwesa(txt)
    # my.sw(txt)
    debug('-----------------------------------------------------------')
    ######################################################################

    ######################################################################
    #mybd = MyBD()
    #mybd.process_json_data(in_fpath, out_fpath, cache_fpath, fail_fpath)
    ######################################################################

    ######################################################################
    #get_last_line()
    ######################################################################

    ######################################################################
    # # 创建任务队列
    # task_queue = Queue()
    #
    # # 填充任务队列
    # with (open(in_fpath, 'r', encoding='utf-8') as file):
    #     debug(f"open {in_fpath} success")
    #     data = json.load(file)
    #     i = 0
    #     for item in data:
    #         debug(f"fill queue with {i}, instruction: {item['instruction']}")
    #         s = json.dumps([item], ensure_ascii=False)
    #         task_queue.put((s, i))
    #         i += 1
    #
    # # 创建线程列表
    # threads = []
    # num_threads = 40  # 设定线程数量
    #
    # # 创建并启动线程
    # for i in range(num_threads):
    #     thread = threading.Thread(target=worker, args=(task_queue,))
    #     thread.start()
    #     threads.append(thread)
    #
    # # 等待队列中的所有任务完成
    # task_queue.join()
    #
    # # 等待所有线程结束
    # for thread in threads:
    #     thread.join()
    #
    # debug('All tasks are completed.')

    ######################################################################
    with open("train_data_final02.json", 'w', encoding='utf-8') as out_file:
        # 存储所有JSON对象的列表
        merged_data = []
        for i in range(45):
            if i < 5:
                continue
            fn = f'./train_data_processed_Thread-{i} (worker).json'
            print(f" open {fn}")
            with open(fn, 'r', encoding='utf-8') as file:
                data = json.load(file)  # 从文件中加载JSON数据
                merged_data.extend(data)  # 将数据添加到merged_data列表中
            print(f" merge {fn} done")
        out_file.write(json.dumps(merged_data, ensure_ascii=False) + '\n')
        out_file.close()
    print(f" merge all done")


if __name__ == '__main__': 
    main()
