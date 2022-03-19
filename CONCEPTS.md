# Concepts

## Utterance

An Utterance is the unit of conversation in the simulation. It consists of a
Predicate, and may have a Subject and/or Object. The Predicate may imply the
Subject, Object, or both.

## Language

Languages are composed of:

- A Vocabulary, representing concepts in the world
- A Grammar, representing the structure and features to convey concepts

Each Agent is assigned one or more languages at the start of the simulation.

## Grammar

A Grammar describes the possible Utterances in a Language. It defines the order
and components of an Utterance, such as:

- Subject; Predicate; Optional<Object>
- SuperPredicate<Subject, Object>

## Vocabulary

A Vocabulary is the set of Words the language recognizes. In these
over-simplified grammars, there are only Predicates and Nouns. A Word is
identified by a number and category (between Predicates and Nouns).

## Simulation

The simulation advances each step by Agents having random conversations with
each other, attempting to have a conversation.
