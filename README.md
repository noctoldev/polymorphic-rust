# polymorphic rust engine

## What’s This About?

Okay, so `RustPolyMorph` is a cool tool I built with Rust that spits out ransomware that works on Windows. I make it on Linux and turn it into a Windows `.exe` file. It grabs files where it’s at, encrypts them with AES-256-CBC, and sticks a `.enc` on the end.

This is gonna tell you all about how it hides and changes itself.

## Polymorphism (aka Shape-Shifting Code)

Polymorphism is just a fancy word for making the code look different every time so antivirus can’t catch it easy. Here’s how I did it:

### 1. Random Names Everywhere
- **What’s Up**: All the names for functions and stuff get random letters and numbers, like `k_X7k9p2` or `ransomware_Blah123`.
- **How**: I’ve got this part that makes up random strings with a mix of letters and numbers.
- **Why**: If the names keep changing, antivirus can’t just look for the same old words. They’ve gotta guess what it’s doing instead!

### 2. Junk Code
- **What’s Up**: I throw in a bunch of useless code that doesn’t do anything real.
- **How**: It makes loops that mess with numbers and waits a random bit before moving on.
- **Why**: It makes the file look different every time and tricks scanners into thinking it’s just some boring program.

### 3. Switching Things Up
- **What’s Up**: It can do stuff in different ways, but right now it’s just one sneaky style.
- **How**: There’s a picker that could choose different tricks, but I’m sticking with one for now.
- **Why**: Changing how it acts could confuse stuff even more, but I’m keeping it simple for this version.
- 
### 1. Hiding in Memory
- **Who It Tricks**: Defender and Symantec (they check files), Elastic (it watches file changes).
- **What’s Up**: It runs its main stuff right in the computer’s memory, not on the hard drive.
- **How**: Uses a Windows trick to grab some memory space and run code there without saving it anywhere.
- **Why**: If it’s not on the disk, those file-scanning dudes can’t find it easy, and it leaves less tracks for Elastic to spot.

### 2. Chillin’ Between Encrypting
- **Who It Tricks**: All of ‘em (they watch how fast stuff happens).
- **What’s Up**: It waits random times (like 50 to 500 milliseconds) before doing each file.
- **How**: Just pauses for a random bit after encrypting something.
- **Why**: Looks more like normal stuff instead of blasting through files super fast, which would totally get caught.

### 3. Mixing Up the Order
- **Who It Tricks**: Elastic (it connects the dots), Symantec (it watches patterns).
- **What’s Up**: Doesn’t encrypt files in order—it shuffles them around first.
- **How**: Grabs all the files in a list and mixes them up before starting.
- **Why**: Stops those smarty-pants systems from seeing a pattern like “oh, it’s doing A, then B, then C.”

### 4. Keeping It Low-Key
- **Who It Tricks**: Elastic (it checks big changes), Defender (it hates tampering).
- **What’s Up**: Doesn’t mess with the system, just encrypts files and chills.
- **How**: Stays away from changing settings or killing other programs.
- **Why**: Less noise means less chance of setting off alarms about messing with security stuff.

## How It Works (Techy Stuff)

### Encryption
- **How It Locks Files**: Uses AES-256-CBC (big brain encryption) with some padding thingy to make it work right.
- **Key and Stuff**: Has a 32-byte key and 16-byte IV that’s all scrambled up and hidden ‘til it runs.
- **What It Does**: Grabs a file, encrypts it in memory, saves it with `.enc`, and trashes the old one.

### Tools I Used
- Some Rust libraries for random stuff, encrypting, and Windows tricks. It’s all set up to work from Linux to Windows.

### Making It
- I build it on Linux and turn it into a Windows executable. Needs a fancy Rust version for some cool low-level bits.

## What’s Not Perfect Yet

### Weak Spots
- **Same IV**: Uses the same IV for every file (oops, not super safe—real ransomware wouldn’t do that).
- **Just One Trick**: Only does the memory thing right now, not tons of different styles.
- **Might Get Caught**: Smart EDR might still notice the memory moves or file renaming.

### Cool Ideas to Add
1. **Sneaky API Stuff**: Grab Windows tools on the fly instead of hard-coding them.
2. **No Debugging Allowed**: Check if someone’s watching it and stop if they are.
3. **Hide the Whole Thing**: Encrypt it all and only unlock it when it runs.
4. **hiding**: Pretend to be a normal program like `svchost.exe`.

## How to Test It

### Setup
- **Where**: A Windows VM with Defender, Symantec, and Elastic running.
- **Files**: Some fake files to encrypt.

### Steps
1. Build my generator on Linux.
2. Make a new ransomware file.
3. Turn it into a Windows `.exe`.
4. Run it in the VM and see what the security stuff does.

## Don’t Be Dumb With This

This is just for learning and testing 
- Only use it where it’s safe and you’re allowed.
- Don’t give it to anyone to mess things up.
- Tell people how to stop this kinda stuff, not how to ruin someone’s day.

## Wrap-Up

`RustPolyMorph` is my way of showing how Rust can make sneaky, changing ransomware that’s tough to catch. It hides in memory, mixes things up, and stays quiet to dodge the big security guys. It’s not perfect, but it’s a cool start for figuring out how to beat malware at its own game

#detections
theres 3 detections as of posting because i was too lazy to evade. crowdstrike, defender, and elastic iirc as f march 14th 2025
