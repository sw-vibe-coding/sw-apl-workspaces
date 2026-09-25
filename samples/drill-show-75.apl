⍝!MODES (B)
⍝ DRILL: the tables and the generator. SHOW prints exercises made at
⍝ random with their answers; the sequence is the same on every load,
⍝ since the workspace's random link is fixed. In (B) there is execute,
⍝ so VERIFY, defined here and not in the workspace, evaluates the
⍝ characters of a hundred more exercises and counts how many agree
⍝ with the value the tables gave: the check that display and value
⍝ are the same exercise.
)LOAD 2 DRILL
DESCRIBE
SHOW 100
∇R←VERIFY N;∆I;∆E;∆V;∆Z
R←0
∆I←0
L:→(∆I≥N)/0
∆I←∆I+1
∆NEW
∆Z←⍎∆E
→((⍴⍴∆Z)≠⍴⍴∆V)/L
→((×/⍴,∆Z)≠×/⍴,∆V)/L
R←R+∧/,(,∆Z)=,∆V
→L
∇
(VERIFY 100);' OF 100 AGREE'
)OFF
