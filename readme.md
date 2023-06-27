This is a sample project on how to build a web service with rust. Resulting from me watching the video at : https://www.youtube.com/watch?v=XZtlD_m59sM

Feel free to reuse it as some little steps are not visible from the video and i had to figure them out.

In one terminal window use: 
```
> cargo watch -q -c -w src/ -x run
```

to run the server code in dev mode so each modification of the sources restarts the server.


In another terminal window, use this to attach the test code so every modification of the test file will run the test again

```
> cargo watch -q -c -w tests -x "test -q quick_dev -- --nocapture"
```