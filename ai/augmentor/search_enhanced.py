"""Enhanced search module"""

import logging
import re
from typing import List, Dict, Optional, Any, Callable, Set
from dataclasses import dataclass, field
from collections import defaultdict

logger = logging.getLogger(__name__)

@dataclass
class SearchResult:
    items: List[Dict]
    total_matches: int
    query: str
    method: str
    highlights: List[Dict] = field(default_factory=list)

    def to_dict(self) -> Dict:
        return {
            "items": self.items,
            "total_matches": self.total_matches,
            "query": self.query,
            "method": self.method,
        }

@dataclass
class SearchFilter:
    field: str
    operator: str
    value: Any

    def to_dict(self) -> Dict:
        return {"field": self.field, "operator": self.operator, "value": self.value}

class EnhancedSearcher:
    def __init__(self, items: List[Dict] = None):
        self._items = items or []
        self._indexes: Dict[str, Dict[str, List[int]]] = {}
        if items:
            self.load(items)

    def load(self, items: List[Dict]):
        self._items = items
        self._build_indexes()

    def _build_indexes(self):
        self._indexes.clear()
        for idx, item in enumerate(self._items):
            for field, value in item.items():
                if isinstance(value, str):
                    if field not in self._indexes:
                        self._indexes[field] = defaultdict(list)
                    self._indexes[field][value.lower()].append(idx)

    def search(self, query: str, fields: List[str] = None,
               method: str = "contains", filters: List[SearchFilter] = None,
               limit: int = 100, offset: int = 0) -> SearchResult:
        fields = fields or ["instruction", "output", "input"]
        results = {}
        for field in fields:
            if method == "contains":
                results.update(self._search_contains(field, query))
        sorted_items = sorted(results.keys())
        paginated = sorted_items[offset:offset + limit]
        highlights = []
        result_items = [self._items[idx] for idx in paginated]
        return SearchResult(
            items=result_items,
            total_matches=len(sorted_items),
            query=query,
            method=method,
            highlights=highlights,
        )

    def _search_contains(self, field: str, query: str) -> Dict[int, float]:
        matches = {}
        query_lower = query.lower()
        for idx, item in enumerate(self._items):
            value = item.get(field, "")
            if isinstance(value, str) and query_lower in value.lower():
                matches[idx] = 1.0
        return matches

    def get_statistics(self) -> Dict:
        return {
            "total_items": len(self._items),
            "indexed_fields": list(self._indexes.keys()),
            "field_counts": {field: len(values) for field, values in self._indexes.items()},
        }


def search_dataset(items: List[Dict], query: str,
                   fields: List[str] = None,
                   method: str = "contains",
                   limit: int = 100) -> SearchResult:
    return EnhancedSearcher(items).search(query, fields, method, limit=limit)


def create_searcher(items: List[Dict] = None) -> EnhancedSearcher:
    return EnhancedSearcher(items)

__all__ = [
    "EnhancedSearcher",
    "SearchResult",
    "SearchFilter",
    "search_dataset",
    "create_searcher",
]
