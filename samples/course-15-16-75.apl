⍝!MODES (B)
⍝ COURSE lessons 15 and 16: workspaces, and the two modes. Lesson 15
⍝ renames, saves, lists, copies from library 2 and lists the
⍝ libraries, all at the pauses; after it, the saved copy is loaded
⍝ and dropped. Lesson 16 tries each mode's own features, so half of
⍝ them are errors whichever mode this runs in, which is the point.
⍝ The (B) twin saves under another name, so the two can run at once.
)LOAD 2 COURSE
LESSON 15
)WSID
)VARS
)WSID MINEB
GO
)SAVE
)LIB
GO
)ERASE HELLO
)COPY 2 COURSE HELLO
HELLO
GO
)LIBS
)LIB 2
GO
QUIZ 15
0
')COPY'
')CLEAR'
'CLEAR WS'
'DESCRIBE'
)LOAD MINEB
)WSID
)DROP MINEB
)LIB
LESSON 16
)DIALECT
GO
'DATE (VARIES): ';⌶25
)DIGITS 5
2÷3
)DIGITS 10
GO
⍎'2+2'
⎕PP←5
2÷3
⎕PP←10
⎕TS
GO
QUIZ 16
'A'
'B'
4
1900
')WSID'
⍝ After the last lesson, NEXT has nowhere to go.
NEXT
)OFF
