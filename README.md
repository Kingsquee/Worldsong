Worldsong
=========

Worldsong is a framework that enables safe, easy runtime editing of your application's behaviour, without sacrificing performance or state data integrity.


Quickstart
----------

1. Navigate to tools/scripts/os_setup.
2. Run the script for your OS. (Currently only gnu/linux)
3. When it finishes, you can run _./launch_ in the worldsong root directory.
4. While it's running, try editing processes/graphics/graphics.rs, and run _./compile_ in the same directory.
5. Refocus the Worldsong window.
6. Smile!

Each subsystem has its own custom _compile_.

To add your own functionality:

1. Run 'add --name yourstructname --editor youreditor' in the structs directory
2. Add some data definitions.
3. Run 'add --name yourprocessname --editor youreditor' in the processes directory
4. Write some code that modifies yourstructname
5. Add your process to the appropriate schedule, passing it yourstructname.

To use third-party libraries, list the dependencies yourstructname requires in its Dependencies.toml, using the format Cargo.toml expects.

Have fun!


What it's made of
-----------------

#####State Structs
Modules that describe state data.

#####State Library
A library automatically generated from state structs, and their requested dependencies.

#####Kernel:
Initializes the state data, hotloads the scheduler, and sends the state data to it.

#####Scheduler:
The main loop. Sets the conditions and timing for when schedules should be run.

#####Schedule(s):
List which processes should be run, and in what order, when the schedule is called.

#####Processes:
Modify state data. These babies do the actual work!

#####Common Library:
Stores data and functionality common to other subsystems. Boring API stuff.

*In practice, it works something like this:*

>![It's something like this, anyway.](http://i.imgur.com/Rac2pZq.png)


Questions and Skepticism
------------------------

#####*"Why did you write this?"*
I used to use Unity, but the complicated, fickle state preservation of their hotloading bothered me. I wanted a framework that was reliable, performant, easy to understand, and capable of being used for any kind of software project.

#####*"So it's an abstracted main loop?"*
An abstracted, hotloadable, state-preserving main loop!

#####*"This isn't parallelizable."*
Totally is. It was designed to be used with some form of parallel job execution, like [this](https://github.com/mcpherrinm/parallel).

#####*"Isn't all data technically global?"*
Schedules define what state a process can access, so unintended side effects would require exceptionally bad coding practices and probably copious amounts of alcohol.

#####*"State data layout can't be modified at runtime!"*
That would invalidate the state.

#####*"Why GPL?"*
Because I'm a nice guy. If you've got a license idea that's not MIT's anarchy or closed-source's despotism, hit me up!

