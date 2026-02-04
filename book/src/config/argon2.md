# Password Hashing Setup

Even though the options and tools in the Admin UI should be fully documented, I wanted to mention
argon2id tuning here.

Rauthy uses the argon2id hashing algorithm for passwords. This is the most expensive and
compute-heavy operation done by the application, and the variables need to be tuned for every
deployment to provide the best compromise of security, resource usage and user experience.  
The default values are a good start at the lower end of the "being safe scale" for a production
deployment. However, you should at least take a look at them and verify that they work for you.

The Admin UI provides a utility which helps you find the best values for your deployment quickly.
What and how to do is described in the Admin UI itself. I just want to guide you to this utility,
especially since it is an important step security wise.

When you are logged in to the Admin UI, please navigate to `Config` -> `Argon2 Parameters` to find
your values. After they have been found, apply them to the Rauthy config and restart the
deployment. \
Keep in mind that if you run the application in a way, where memory is limited, for instance inside
Kubernetes with resource limits set too low, that it will crash, if either `hashing.argon2_m_cost`
is set too high or the memory limit too low.

There is one additional, really important config variable need to be taken into account for the
tuning. \
Since this operation is really resource intense, you can limit the number of threads which can run
in parallel doing hashing operations. This is really important when we think about constrained
memory again.

`hashing.max_hash_threads` limits the maximum number of parallel password hashes at the exact same
time to never exceed system memory while still allowing a good amount of memory. The **default value
is 2**.

The rule is simple: Allow as many resources as possible for hashing to have the maximum amount of
security, while restricting it as much as necessary.

For smaller deployments, set `hashing.max_hash_threads`, which will technically allows only one user
login at the exact same time. This value makes an external rate limiting for the login obsolete (
while you may add some for the others).
