# rustbatch

This is my attempt to make 2D game library in rust. My main focus is performance and bibrary so far provides fast solutions for some bottle necks of games like collizion
detection or path finding. Though main feature of library is opengl wrapper build around batching. Check out [examples repository]https://github.com/jakubDoka/rustbatch_examples
where i demonstrated capabilities on rendering and processing 10k boids with 60 fps. With a help of rustbatch Scanner there is noo need to make 100 milion interations everi 
frame.

If you are wondering why are first tree versions of rustbatch yanked, lets say i did not test core featires sufficiently.
