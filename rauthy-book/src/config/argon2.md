# Argon 2 Setup

Even though the options and tools in the Admin UI should be fully documented, I wanted to mention argon2id tuning here
anyway.

Rauthy uses the argon2id hashing algorithm for passwords. This is the most expensive and compute-heavy operation
beging done by the application and the variables need to be tuned for every deployment to provide the best compromise
of security, resource usage and user experience.  
The default values are way too low for a real production deployment. They should only be used for testing.

The Admin UI provides a utility which helps you find the values for your deployment quickly. What and how to do is 
described in the Admin UI itself, I just want to guide you to this utility especially, since it is an important step
security wise.

When you are logged in to the Admin UI, please navigate to `Config` -> `Argon2 Parameters` to find your values.  
After they have been found, apply them to the rauthy config and restart the deployment.  
Keep in mind, that if you run the application in a way, where memory is limited, for instance inside Kubernetes with
resource limits set too low, that it will crash, if either `ARGON2_M_COST` is set too high or the memory limit too low.

There is one additional, really important config variable need to be taken into account for the tuning.  
Since this operation is really ressource intense, you can limit the amount of threads, which can run in parallel doing
hashing operations. This is really important when we think about constrained memory again. 

`MAX_HASH_THREADS` limits the maximum amount of parallel password hashes at the exact same time to never exceed system
memory while still allowing a good amount of memory.  
The **default** value **is 2**.

The rule is simple: Allow as many resources as possible for hashing to have the maximum amount of security, while 
restricting it as much as necessary.

For smaller deployments, set `MAX_HASH_THREADS=1`, which will technically allows only one user login at the exact same
time. This value makes an external rate limiting for the login obsolete (while you may add some for the others).
