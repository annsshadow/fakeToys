"""对话上下文增强模块 - 优化版"""

import json
import logging
from typing import List, Dict, Optional
from concurrent.futures import ThreadPoolExecutor, as_completed
from .models.base import ModelBackend

logger = logging.getLogger(__name__)


class ContextAugmentor:
    """对话上下文增强器 - 优化版"""
    
    def __init__(self, 
                 model_backend: ModelBackend,
                 num_turns: int = 3,
                 max_workers: int = 5):
        """初始化对话上下文增强器
        
        Args:
            model_backend: 模型后端
            num_turns: 对话轮数
            max_workers: 最大并发数
        """
        self.model_backend = model_backend
        self.num_turns = num_turns
        self.max_workers = max_workers
    
    def _generate_follow_up_question(self, 
                                    history: List[Dict],
                                    current_question: str,
                                    current_answer: str) -> str:
        """生成后续问题
        
        Args:
            history: 历史对话
            current_question: 当前问题
            current_answer: 当前回答
        
        Returns:
            生成的后续问题
        """
        # 构建上下文
        context_parts = []
        for turn in history:
            role = "用户" if turn["role"] == "user" else "客服"
            context_parts.append(f"{role}: {turn['content']}")
        
        context_parts.append(f"用户: {current_question}")
        context_parts.append(f"客服: {current_answer}")
        
        context_str = "\n".join(context_parts)
        
        prompt = f"""基于以下对话，生成一个自然、相关的后续问题。

对话历史：
{context_str}

请生成一个用户可能会问的后续问题，要求：
1. 与当前对话主题相关
2. 自然流畅，像真实用户会问的问题
3. 不要重复已问过的问题
4. 只输出问题本身，不要任何解释

后续问题："""
        
        try:
            response = self.model_backend.generate(prompt)
            # 清理响应，只保留问题部分
            question = response.strip()
            # 移除可能的前缀
            for prefix in ["后续问题：", "问题：", "Q：", "A："]:
                if question.startswith(prefix):
                    question = question[len(prefix):].strip()
            return question
        except Exception as e:
            logger.warning(f"生成后续问题失败: {e}")
            return ""
    
    def generate_multi_turn(self, 
                           seed_qa: Dict,
                           existing_histories: Optional[List[List[Dict]]] = None) -> Dict:
        """生成多轮对话数据
        
        Args:
            seed_qa: 种子问答对 {"instruction": "...", "output": "..."}
            existing_histories: 已有的历史对话（用于去重）
        
        Returns:
            多轮对话数据
        """
        history = []
        current_q = seed_qa.get("instruction", "")
        current_a = seed_qa.get("output", "")
        
        # 生成后续对话
        for i in range(self.num_turns - 1):
            follow_up = self._generate_follow_up_question(
                history, current_q, current_a
            )
            
            if not follow_up:
                break
            
            # 检查是否与已有历史重复
            if existing_histories:
                is_duplicate = False
                for existing in existing_histories:
                    existing_questions = [t["content"] for t in existing if t["role"] == "user"]
                    if follow_up in existing_questions:
                        is_duplicate = True
                        break
                if is_duplicate:
                    continue
            
            history.append({"role": "user", "content": current_q})
            history.append({"role": "assistant", "content": current_a})
            
            current_q = follow_up
            # 回答保持不变（实际场景中可能需要重新生成）
        
        # 构建最终数据
        result = {
            "instruction": current_q,
            "input": seed_qa.get("input", ""),
            "output": current_a,
            "history": history,
            "system": seed_qa.get("system", "你是安居乐寓的智能客服")
        }
        
        return result
    
    def batch_generate(self,
                      items: List[Dict],
                      existing_histories: Optional[List[List[Dict]]] = None,
                      use_parallel: bool = True) -> List[Dict]:
        """批量生成多轮对话数据（优化版）
        
        Args:
            items: 种子问答对列表
            existing_histories: 已有的历史对话
            use_parallel: 是否使用并行处理
        
        Returns:
            多轮对话数据列表
        """
        if not use_parallel or len(items) <= 1:
            # 串行处理
            results = []
            for i, item in enumerate(items):
                logger.info(f"生成多轮对话 {i + 1}/{len(items)}")
                multi_turn = self.generate_multi_turn(item, existing_histories)
                results.append(multi_turn)
            return results
        
        # 并行处理
        results = [None] * len(items)
        
        def process_item(idx_item):
            idx, item = idx_item
            return idx, self.generate_multi_turn(item, existing_histories)
        
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            futures = [executor.submit(process_item, (i, item)) for i, item in enumerate(items)]
            
            for future in as_completed(futures):
                try:
                    idx, result = future.result()
                    results[idx] = result
                    logger.info(f"完成多轮对话 {idx + 1}/{len(items)}")
                except Exception as e:
                    logger.error(f"生成多轮对话失败: {e}")
        
        return [r for r in results if r is not None]
    
    def convert_single_to_multi_turn(self,
                                    items: List[Dict],
                                    num_turns: Optional[int] = None) -> List[Dict]:
        """将单轮对话数据转换为多轮对话数据
        
        Args:
            items: 单轮对话数据列表
            num_turns: 对话轮数，为 None 时使用默认值
        
        Returns:
            多轮对话数据列表
        """
        if num_turns is not None:
            original_num_turns = self.num_turns
            self.num_turns = num_turns
        
        try:
            results = self.batch_generate(items)
            return results
        finally:
            if num_turns is not None:
                self.num_turns = original_num_turns
    
    def add_history_to_single_turn(self,
                                  items: List[Dict],
                                  history_length: int = 2) -> List[Dict]:
        """为单轮对话添加历史上下文（不改变原始问答）
        
        Args:
            items: 单轮对话数据列表
            history_length: 历史长度
        
        Returns:
            添加了历史上下文的数据列表
        """
        results = []
        
        for i, item in enumerate(items):
            # 从前面的数据中随机选择作为历史
            history = []
            start_idx = max(0, i - history_length)
            
            for j in range(start_idx, i):
                history.append({"role": "user", "content": items[j]["instruction"]})
                history.append({"role": "assistant", "content": items[j]["output"]})
            
            result = {
                "instruction": item["instruction"],
                "input": item.get("input", ""),
                "output": item["output"],
                "history": history,
                "system": item.get("system", "你是安居乐寓的智能客服")
            }
            results.append(result)
        
        return results
