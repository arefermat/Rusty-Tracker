# Rusty-Tracker
A todo tracker that lists and tracks current, past, and future assignments under subjects. A very simple, efficient and fast CLI using Rust.

## Contact and Bugs
If you find any bugs of any kind, please let me know or open an issue with a descriptive name and detials about the bug.

## Commands

### help
Use this command when first trying out, prints commands and helps you figure our how to use commands.
#### Note
You can also use cmdhelp COMMAND to see details on that specific command.

### new
This command creates a new assignment assimged to an subject with a due date and an estimated time to complete.
#### Subcommands
1. **Assignment name** : enter a unique name for the assignment
2. **Due date** : Enter a due date however you like 
3. **Status** : whether the assignment is "done" or "incomplete"

**Example** : new Assignment April_5th incomplete


### edit
This command lets you edit existing assignments, including their name and due_date.
#### Subcommands
1. **Assignment name** : enter an existing assignment name to target
2. **What to Change** : what part of the assignment you want to change (either "name" or "due_date")
3. **Change to** : what you want to change the detail to

**Example** : edit Assignment due_date Aprilt_6th

### mark
This command allows you to change and choose whether a specific assignment is marked done or incomplete.
#### Subcommands
1. **Assignment name** : enter an existing assignment name to target
2. **New status** : enter either "done" or "incomplete" to change the status of the assignment targeted

**Example** : mark Assignment done

### view
This command lets you view all existing assignments.
#### Subcommands
**None** : this command contains no subcommands

**Example** : view

### remove
This command allows you to remove entire assignments, beware, these changes are permanent.
#### Subcommands
1. **Assignment name** : enter an existing assignment name to target

**Example** : remove Assignment



