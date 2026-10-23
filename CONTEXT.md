# Fasteyes

A speed reader that presents words from a document.

## Language

**Document**:
The input text being read.

**Word**:
A whitespace-separated token, with punctuation retained.

**Words per minute (WPM)**:
The reading rate measured in words presented per minute.

**Words per display**:
The number of words presented together at one time.

**Display group**:
The consecutive words shown together at the current reading position. The
final group may contain fewer words than the selected words per display.

**Reading position**:
The first word of the current display group.

**Seeking**:
Moving the reading position to a chosen word in the document.

**Blank state**:
The reader has no document loaded and no display group to present.

**Group navigation**:
Seeking backward or forward by the selected words per display from the current
reading position, stopping at the document boundaries.
