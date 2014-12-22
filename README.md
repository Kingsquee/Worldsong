Worldsong
=========

Write code. Hit compile. Tab into the running application. See your changes updated, running at native speed. Smile as your state data isn't decimated in the process.

This is the dream of Worldsong.


What it's made of
-----------------

#####Common Library:
Stores data and functionality common to other subsystems, including as state data and third-party libraries.

#####Kernel:
Initializes the state data, hotloads the scheduler, and sends the state data to it.

#####Scheduler:
The main loop. Sets the conditions and timing for when schedules should be run.

#####Schedule(s):
List which processes should be run, in order, when the schedule is called.

#####Process(es):
Modify state data. These babies do the actual work.

In practice, it works something like this:
![It's something like this, anyway.](http://i.imgur.com/PZJEnhB.png)


Questions and Skepticism
------------------------

#####"Why did you write this?"
I used to use Unity, but the garbage collected languages and the fickle state preservation of hotloading bothered me. Plus, no source code.
I wanted a framework that let me use one language, with state-preserving runtime editing, with no overhead.

#####"So it's an abstracted main loop?"
An abstracted, hotloadable, state-preserving main loop!

#####"This isn't parallelizable."
Totally is. It was designed to be used with some form of [parallel job execution](https://github.com/mcpherrinm/parallel).

#####"This isn't concurrent!"
That's the idea, yeah.

#####"Isn't all data technically global?"
Schedules define what state a process can access, so unintended side effects would require exceptionally bad coding practices and probably copious amounts of alcohol.

#####"State data layout can't be modified at runtime!"
No, because that would invalidate the state. We *could* make it work though, see below.


TODO / What can be improved?
----------------------------

#####Live reloading of state data.
Closing the program to reset the state is awkward. It'd be nice if state::Data was hotloadable at runtime.
The problem is, what subsystem owns the state::Data instance? What ungodly unsafe block could access functions of something it doesn't have type information for?

Assuming the below isn't implemented, the cleanest way to achieve this would be to write an OS-level call to restart the application.

#####Live _editing_ of state data's layout.
Ownership of unknown types. Built-in serialization. Conversion function hooks on load. We go full erlang.

PRs are welcome.
