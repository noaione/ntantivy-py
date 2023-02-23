"""
MIT License

Copyright (c) 2019 The Matrix.org Foundation CIC

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""

# TODO: Add docstrings for all the classes and methods.

from datetime import datetime
from typing import Any, Generic, Literal, Optional, TypeAlias, TypeVar

# Private types, cannot be imported from tantivy itself.
_TokenizerType: TypeAlias = Literal["raw", "whitespace", "default", "en_stem"]
_IndexOptions: TypeAlias = Literal["position", "basic", "freq"]
_ReloadPolicyOptions: TypeAlias = Literal["oncommit", "manual"]
_OpStamp: TypeAlias = int
_Cardinality: TypeAlias = Literal["single", "multi"]
_SchemaT = TypeVar("_SchemaT", bound=Schema)

class Schema(Generic[_SchemaT]): ...

class SchemaBuilder(Generic[_SchemaT]):
    def __init__(self) -> None: ...
    def add_text_field(
        self, name: str, stored: bool = ..., tokenizer_opts: _TokenizerType = ..., index_option: _IndexOptions = ...
    ) -> None: ...
    def add_integer_field(
        self, name: str, stored: bool = ..., indexed: bool = ..., fast: _Cardinality | None = ...
    ) -> None: ...
    def add_unsigned_field(
        self, name: str, stored: bool = ..., indexed: bool = ..., fast: _Cardinality | None = ...
    ): ...
    def add_float_field(
        self, name: str, stored: bool = ..., indexed: bool = ..., fast: _Cardinality | None = ...
    ): ...
    def add_bool_field(
        self, name: str, stored: bool = ..., indexed: bool = ..., fast: _Cardinality | None = ...
    ): ...
    def add_date_field(self, name: str, stored: bool = ..., indexed: bool = ..., fast: _Cardinality | None = ...): ...
    def add_json_field(
        self, name: str, stored: bool = ..., tokenizer_opts: _TokenizerType = ..., index_option: _IndexOptions = ...
    ): ...
    def add_facet_field(self, name: str): ...
    def add_bytes_field(self, name: str): ...
    def build(self) -> Schema[_SchemaT]: ...

class Facet:
    @classmethod
    def root(cls) -> Facet: ...
    @property
    def is_root(self) -> bool: ...
    def is_prefix_of(self, other: Facet) -> bool: ...
    @classmethod
    def from_string(cls, facet_string: str) -> Facet: ...
    def to_path(self) -> list[str]: ...
    def to_path_str(self) -> str: ...
    def __repr__(self) -> str: ...

class Document:
    def __init__(self, default_tokenizer: _TokenizerType = ..., **kwargs) -> None: ...
    def extend(self, **kwargs) -> None: ...
    @staticmethod
    def from_dict(data: dict[str, Any], default_tokenizer: _TokenizerType = ...) -> Document: ...
    def to_dict(self) -> dict[str, Any]: ...
    def add_text(self, field_name: str, text: str, opts: _TokenizerType = ...) -> None: ...
    def add_unsigned(self, field_name: str, value: int) -> None: ...
    def add_integer(self, field_name: str, value: int) -> None: ...
    def add_float(self, field_name: str, value: float) -> None: ...
    def add_bool(self, field_name: str, value: bool) -> None: ...
    def add_date(self, field_name: str, value: datetime) -> None: ...
    def add_facet(self, field_name: str, value: Facet) -> None: ...
    def add_bytes(self, field_name: str, value: bytes) -> None: ...
    def add_json(self, field_name: str, value: str) -> None: ...
    @property
    def num_fields(self) -> int: ...
    @property
    def is_empty(self) -> bool: ...
    def get_first(self, field_name: str) -> Any: ...
    def get_all(self, field_name: str) -> list[Any]: ...
    def __getitem__(self, field_name: str) -> list[Any]: ...
    def __repr__(self) -> str: ...

class _IndexWriter:
    def add_document(self, document: Document) -> _OpStamp: ...
    def add_json(self, json: str) -> _OpStamp: ...
    def commit(self) -> _OpStamp: ...
    def rollback(self) -> _OpStamp: ...
    def delete_documents(self, field_name: str, field_value: Any) -> _OpStamp: ...
    def garbage_collect_files(self) -> None: ...
    @property
    def commit_opstamp(self) -> _OpStamp: ...

class Query:
    def __repr__(self) -> str: ...

class DocAddress:
    @property
    def segment_ord(self) -> int: ...
    @property
    def doc(self) -> int: ...

class SearchResult:
    @property
    def hits(self) -> list[tuple[int, DocAddress]]: ...
    @property
    def count(self) -> int: ...
    def __repr__(self) -> str: ...

class Searcher:
    def search(
        self, query: Query, limit: int = ..., count: bool = ..., order_by_field: Optional[str] = ..., offset: int = ...
    ) -> SearchResult: ...
    @property
    def num_docs(self) -> int: ...
    def doc(self, doc_address: DocAddress) -> Document: ...
    def __repr__(self) -> str: ...

class Index:
    def __init__(self, schema: Schema, path: str | None = ..., reuse: bool = ...) -> None: ...
    @staticmethod
    def open(path: str) -> Index: ...
    def writer(self, heap_size: int = ..., num_threads: int = ...) -> _IndexWriter: ...
    def config_reader(self, reload_policy: _ReloadPolicyOptions = ..., num_warmers: int = ...) -> None: ...
    def searcher(self) -> Searcher: ...
    @staticmethod
    def exists(path: str) -> bool: ...
    @property
    def schema(self) -> Schema: ...
    def reload(self) -> None: ...
    def parse_query(self, query: str, default_field_names: list[str] | None) -> Query: ...
