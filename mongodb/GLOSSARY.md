# MongoDB Local Application Development Glossary

The canonical terms for the local MongoDB learning labs.

## Core data model

**Document**:
A single BSON record made of field-and-value pairs, such as Ada's learning information.
_Avoid_: Row, data object

**Collection**:
A named group of related documents, such as the student documents in `lesson_01_students`.
_Avoid_: Table, folder

**Database**:
A named group of collections, such as `mongodb_learning`.
_Avoid_: Server, cluster
