
Worldsong
=========

Worldsong is a framework that enables safe, easy runtime editing of your application's behaviour, without sacrificing performance or state data integrity.

Quickstart
----------

1. Run 'cargo run --release' in the setup directory.
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

#####/state/
State data (instances of types stored on the heap) are described in here. Processes modify these.
Changes made here requires a program restart (F5)

#####/types/ (Optional)
Types used throughout the program, but don't deserve a third party library.
Changes made here requires a program restart (F5)

#####/kernel/
Initializes the state data, hotloads the scheduler, and sends it a reference to the state library.
Changes made here requires a program restart (F5)

#####/scheduler/
The main loop. Sets the conditions and timing for when schedules should be run.
Changes made here can be hotloaded at runtime! (F4)

#####/schedules/
Simple lists of which processes should be run, and in what order, when the schedule is called. Used for things like variable or fixed update loops.
Changes made here can be hotloaded at runtime! (F4)

#####/processes/
Small groups of functions that modify state data.
Changes made here can be hotloaded at runtime! (F4)

Of course, you can add your own directores as you see fit - /resources/, /shaders/, etc.

*In practice, it works something like this:*

>![It's something like this, anyway.](http://i.imgur.com/Rac2pZq.png)


Questions and Skepticism
------------------------

#####*"Why did you write this?"*
I used to use Unity, but the complicated, fickle state preservation of their hotloading bothered me. I wanted a framework that was reliable, performant, easy to understand, and capable of being used for any kind of software project.

#####*"So it's an abstracted main loop?"*
An abstracted, hotloadable, state-preserving main loop!

#####*"Shouldn't this be threaded up the ying yang like all the cool kids do?"*
Sure, but I'm not making that decision for you. It was designed to be used with some form of parallel job execution, like [this](https://github.com/mcpherrinm/parallel). The _/scheduler/_ is your playground, my friend.

#####*"Isn't all data technically global?"*
_/schedules/_ define what state a process can access, so unintended side effects would require exceptionally bad coding practices and probably copious amounts of alcohol.

#####*"State data can't be modified at runtime!"*
Well yeah, that would invalidate the state. Go cry in your erlang-flavoured beer.

#####*"GPL!??!?"*
Because I'm a nice guy. If you've got a license idea that's not MIT's anarchy or closed-source's despotism, hit me up!


