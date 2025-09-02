# rust zmq message queue which has different topics/subscribers and can store the messages sent over a local db written to disc?

## GOALS 
- Would liek to make a message system that incorperates zmq in rust. 
- macro test
- this is a test
- this would entail at least 3 different publishers/subscribers 
- where at least 1 subscriber is sub to all possible publishers
- and one publisher is subscribed to no one. (act as an admin pub of sorts) 

- would like to make a make file, or look into cmake for this, and find a way to make it portable
- would like to use config files to create some basis for this
- for now would like to kiss with no gui/with maybe basic cli/backend server (rest?) 
