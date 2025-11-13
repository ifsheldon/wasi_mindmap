"""
See https://github.com/bytecodealliance/cargo-component/issues/360 for why this is needed
Also according to https://component-model.bytecodealliance.org/creating-and-consuming/composing.html, "Composition happens at the level of interfaces"
"""

from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some
