pub const CREATE_WORKFLOW: &'static str = r#"
from typing import Literal

from core.{context}.application.repository import {repository}
from core.shared.errors.domain import UnexpectedError
from dependency_injector.wiring import Provide, inject
from framework.dependency import App
from framework.events.dispatcher import Dispatch
from framework.exceptions import DomainError
from framework.marks import workflow
from framework.shared.python.functions import pair_side_effect
from framework.types import Command, EntityRepr, Event, EventData
from result import Err, Ok, Result


class _EventData(EventData):
    pass


class {command}(Command):
    pass


class {event}(Event):
    aggregate_id: {entity_title_case}ID
    data: dict[Literal["data"], _EventData]


_WFT = tuple[{entity_title_case}, {command}]


def _transit_state(
    command: Create{entity_title_case},
) -> Result[_WFT, DomainError]:
    {entity_snake_case} = {entity_title_case}(
        id=command._entity_id or {entity_title_case}ID.generate(),
    )

    return Ok(({entity_snake_case}, command))


@inject
def _revert_state(
    event: {event},
    repository: {repository} = Provide[App.{context}.repository],
) -> None:
    repository.remove_{entity_snake_case}(event.aggregate_id)


@inject
def _dispatch_event(
    data: _WFT,
    dispatch: Dispatch = Provide[App.event_handler.event_dispatcher],
) -> Result[{event}, DomainError]:
    ({entity_snake_case}, command) = data

    event = {event}(
        aggregate_id={entity_snake_case}.id,
        process_id=command.process_id,
        data=\{"data": _EventData()},
        for_context=None,
        user_id=command._user_id,
    )

    match dispatch(event):
        case Ok():
            return Ok(event)
        case Err():
            _revert_state(event)
            return UnexpectedError().into_err()


@inject
def _assert_invariant(command: {command}) -> Result[{command}, DomainError]:
    return Ok(command)


@workflow("{context}", private={is_private})
def {workflow}(
    command: {command},
    repository: {repository} = Provide[App.{context}.repository],
) -> Result[{event}, DomainError]:
    _save = pair_side_effect({entity_title_case}, {command}, repository.save_{entity_snake_case}
    return (
        _assert_invariant(command)
        .and_then(_transit_state)
        .and_then(_save)
        .and_then(_dispatch_event)
    )
"#;

pub const UPDATE_WORKFLOW: &'static str = r#"

from typing import Literal, assert_never

from core.{context}.application.repository import {repository}
from core.shared.errors.domain import (
    DomainRuleFailure,
    DomainRuleSuccess,
    UnexpectedError,
)
from dependency_injector.wiring import Provide, inject
from framework.dependency import App
from framework.events.dispatcher import Dispatch
from framework.exceptions import DomainError
from framework.marks import workflow
from framework.shared.python.functions import pair_side_effect
from framework.types import Command, Event
from result import Err, Ok, Result


class {command}(Command):
    {entity_snake_case}_id: {entity_title_case}ID


class {event}(Event):
    aggregate_id: {entity_title_case}ID
    data: dict[Literal["old", "new"], {entity_title_case}]


_WFT = tuple[{entity_title_case}, {command}]


def _transit_state(
    data: _WFT,
) -> Result[tuple[{entity_title_case}, {entity_title_case}], DomainError,]:
    old_state, command = data

    new_state = {entity_title_case}(
        id=old_state.id,
    )

    return Ok((new_state, old_state))



@inject
def _revert_state(
    event: {event},
    repository: {repository} = Provide[App.{context}.repository],
) -> None:
    repository.update_{entity_snake_case}(event.data["old"])


@inject
def _dispatch_event(
    data: tuple[{entity_title_case}, {entity_title_case}],
    dispatch: Dispatch = Provide[App.event_handler.event_dispatcher],
) -> Result[{event}, DomainError]:
    new_state, old_state = data

    event = {event}(
        aggregate_id=new_state.id,
        data=\{"new": new_state, "old": old_state},
    )

    match dispatch(event):
        case Ok():
            return Ok(event)
        case Err():
            _revert_state(event)
            return UnexpectedError().into_err()


@inject
def _assert_invariant(
    command: {command},
) -> Result[_WFT, DomainError]:
    invariant = {entity_snake_case}_exists(command.{entity_snake_case}_id)

    match invariant:
        case DomainRuleSuccess(obj):
            return Ok((obj, command))
        case DomainRuleFailure(e):
            return e.into_err()
        case _:
            assert_never(invariant)


@workflow("{context}", private={is_private})
def {workflow}(
    command: {command},
    repository: {repository} = Provide[App.{context}.repository],
) -> Result[{event}, DomainError]:
    _update = pair_side_effect({entity_title_case}, {entity_title_case}, repository.update_{entity_snake_case})
    return (
        _assert_invariant(command)
        .and_then(_transit_state)
        .and_then(_update)
        .and_then(_dispatch_event)
    )
"#;
