
Worldsong
=========

Worldsong is a framework that enables safe, easy, and _fast_ runtime editing of your application's behaviour, without sacrificing performance or state data integrity. 

We do this by maintaining a hard separation between data and process, and further splitting these into the smallest compilation crates that make sense. This allows anything on the 'process' side to be changed with minimal compilation, leaving your application's state data untouched.

Quickstart
----------

1. Run _cargo run --release_ in the setup directory.
2. Navigate to the project of your choice - lets say, _projects/triangle_ - and run _./launch_.
3. While it's running, try editing _projects/triangle/src/processes/graphics_process.rs_.
4. Run _./compile_ in the same directory.
5. Refocus the running application and hit F4.
6. Smile!

Worldsong projects are made of various subsystems, each having its own custom _compile_.

To add your own functionality:

1. Run _add --name yourstructname --editor youreditor_ in the state directory
2. Add some state data definitions.
3. Run _add --name yourprocessname --editor youreditor_ in the processes directory
4. Write some code that modifies yourstructname.
5. Add your process to the appropriate schedule, passing it yourstructname.

To use third-party libraries, list the dependencies yourstructname requires in the project's dependencies/Cargo.toml.

Have fun!


A Worldsong project is made of various submodules:
--------------------------------------------------

######Changes made to the following require a program restart (F5):

#####/state/
State data (instances of types stored on the heap) are described in here. Processes modify these.

#####/types/ (Optional)
Types used throughout the program, but don't deserve a third party library.

#####/kernel/
Initializes the state data, hotloads the scheduler, and sends it a reference to the state library. Handles the hotloading.

######Changes made to the following can be hotloaded at runtime (F4):

#####/scheduler/
The main loop. Sets the conditions and timing for when schedules should be run.

#####/schedules/
Simple lists of which processes should be run, and in what order, when the schedule is called. Used for things like variable or fixed update loops.

#####/processes/
Small groups of functions that modify state data.

Of course, you can add your own directores as you see fit - /resources/, /shaders/, etc.

*In practice, it works something like this:*

>![It's something like this, anyway.](http://i.imgur.com/Rac2pZq.png)


Questions and Skepticism
------------------------

#####*"Why did you write this?"*
I used to use Unity, but the complicated, fickle state preservation of their hotloading bothered me. I wanted a framework that was reliable, performant, easy to understand, and capable of being used for any kind of software project. 

Hopefully, I've accomplished that.

#####*"So it's a modular main loop?"*
A modular, hotloadable, state-preserving main loop!

#####*"Isn't all data technically global?"*
_/schedules/_ define what state a process can access, so unintended side effects would require exceptionally bad coding practices and probably copious amounts of alcohol.

#####*"State can't be recompiled at runtime!"*
Well yeah, that would invalidate the state. Go cry in your erlang-flavoured beer.

#####*"GPL!??!?"*
Because I'm a nice guy. If you've got a license idea that's not MIT's anarchy or closed-source's despotism, hit me up!


