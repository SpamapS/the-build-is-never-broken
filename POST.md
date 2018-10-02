When I joined the OpenStack Engineering team at GoDaddy, I want to make it clear, things were not completely terrible. In fact, there were some really amazing things happening.

Most of the things they needed to do over and over were in code, and 99% of that was in a single chain of git repositories that the entire team was aware of and participating in. There was CI in various forms running against PR's opened on repos and building things after stuff landed. In general, most obvious mistakes didn't waste the team's time.

However, there was one really frustrating thing, that nobody seemed to be able to really solve: There was no realistic dynamic development environment. So many repos, so many components, it just wasn't feasible to scale it down entirely. If you had an idea of how something needed to get done, you had a few choices, none of which were fast or fun:

1) You could write a patch that looked kind of right, propose it against the right git repo, get it through review and landed, wait for the artifact builds to produce the needed bits, deploy them to the "dev" environment, and then iterate on that until dev worked the way you wanted, promoting the artifacts to stage and eventually production.

2) You could try to edit the `dev` environment directly, and reverse engineer what you did back in to a patch, and then do step (1) but with at least a bit more confidence.

3) Cry in to your pillow. Go to step 1.

Now, you may be asking "why not Vagrant?", and I'd say that too, except I almost never have had any success with Vagrant or similar. Because it's so different from production deployments, the Vagrant build is almost always broken a little bit in some ways. Also really, because there are so many of these type (1) changes in-flight, often times `master` is a complete shambles, and you end up needing to either rewind to the last known good deploy, or pull somebody else's branch that has a fix in-flight. It's entirely possible to have a great local-dev or even cloud based Vagrant or Vagrant-like tool. But the effort to build it and maintain it are pretty large, and if the only benefit you get is local dev, you have just traded one velocity problem for others, such as change delivery success rate and transparency.

Does this sound familiar? After joining a few DevOps-ish teams over the last few years, it's a common pattern. This isn't a tools problem. The team was sort of 95% puppet, and 5% Ansible, and that 5% was no better or worse in this respect. No, we didn't have to wait for RPMs to be built for Ansible, but because we also could just iterate on our laptops, many in-flight in-dev changes were actually more dangerous than Puppet changes, because they often would be run on production, and then just forgotten, never re-incorporated in to the git repos.

That's not because the team didn't want to be better tested, nor was it because they lacked the capability. They had some pretty heavy hitting Jenkins talent at their disposal to get anything Jenkins could do done. And it's worth noting that there was an attempt to build a "dev on demand" set of Ansible playbooks to get this done. However, as noted above, this might not actually have resulted in a net-positive win.

The problem that many teams face is the same one that my co-workers faced then: The build was often broken and incomplete, whether that was detected or not, because the tools we have for testing are not flexible where they need to be, and as a result, they're not able to be strong when they need to be.

Solving this problem is the crux of Zuul's very existence. Adopting Zuul means any team, whether you're 3 people working on a single repo, or 300 people working across 25 repos, can reap the benefits of an always-deliverable set of repositories and branches.

So how does Zuul do this, and why can't other systems get this done? Well the answer starts with git.

If you're not familiar with git, um.. it's 2018, please spend the next hour learning what it is, and then 5 minutes asking why on earth your build engineers haven't switched.

Most tools try very hard to be somewhat git-stupid. Jenkins comes to mind here. While it has plugins to have it listen for `GitHub` webhooks, and other change management systems, it really doesn't want to be too aware of git. Git is relegated to the same as `curl`: it's the way you fetch the code to do things with.

But that's really not the case. This single-mindedness around git is often where tools sort of give up. Implementers of CI tools seem reluctant to think of what happens after a git commit lands.

But Zuul came from a place where there were very large penalties for landing broken code. OpenStack had an extremely wide scope, and as such, many developers were showing up with code and integrations between the various projects. Having "the build broken" for 2,000 people shines a very bright light on just how important it is that tests work, and that code does what reviewers and coders believe it does.

Because, you see, when you land things in git, they are thusly part of a timeline that did not exist when others pulled code. And when those others land things in their local git repository, and make it work with their local dev tools, there's nothing to say that what's in master will keep working when integrated with those unfinished changes. Add in dependent repos and changes, and the certainty of something that worked today continuing to work if landed tomorrow is very low. It's all a big, messy, eventually consistent, distributed system.

But it doesn't have to be broken all the time, and we don't have to always rebase on master every time it changes "just in case". If we think of a set of git repositories and/or branches as a unit to be tested together, it's a very short bridge to fully testing things together in the form that they will be made available to others, before they are made available to others. Just like you don't send things to production without testing them in stage exactly as they are first, you shouldn't send code to your developers without having first had it tested exactly as it will land.

And once you decide on that being a good thing to do, other items pop in to your head. What about multi-node testing of distributed systems? How can I make it go faster? These are questions that the creators of Zuul faced as well, and solved with simple, straightforward answers that have been proven valuable through the years of OpenStack running its entire development infrastructure on them.

So, at GoDaddy, when it came time to reboot our OpenStack installations a bit, I couldn't think of a better way to leverage Zuul's power than to start with a 5-VM job to deploy a mini-cloud on to and run some tests against it.

I'll be honest, I gave this initiative a 50% chance of working. The deadlines were tight, and the political environment around the products being supported is a bit stressful. I Fully believed that somebody would look at what we were doing and pull the plug in favor of some other more widely accepted tool or more established pattern.

Luckily, nobody had time to say no to Zuul. So we built this 5-VM job up to the point where it deployed an OpenStack control plane the way we wanted to deploy in production. Our first few deploys to the POC environment found all the ways that a mini-cloud is different than a real one, but we had an enormous amount of momentum built up behind Zuul and this job, and Zuul mostly seemed to be getting out of peoples' way at the right time. We had our "dev on demand", and even better, we had a single, transparent way to land changes.

Since then we've been pushing changes at a pretty impressive clip, and it is quite rare for a change to break production. We do get quite a bit of coverage for our automation from the various Zuul jobs that run before changes land. Mostly those corner cases, such as scale problems, mis-typed config details, or scheduling issues where the hardware actually matters, regularly cause us issues.

But we have devised a scheme to leverage Zuul for these issues as well, by using it to kick off deploys to our staging environment and promote changes from there to production only after automated tests have run.

So how does Zuul do this, and why does it leverage Ansible?

First and foremost, by being fully git-aware, an engineer is effetively able to build a new future in a set of git repositories and branches. If this new future must span multiple repos, Zuul offers the engineer the ability to specify dependencies on other changes.

So, if you need to propose a new variable for a role that is shared amongst automation concerns, and then depend on that variable for yours, you can add `Depends-On: https://your.github/roles_org/role_repo/pulls/1234` to the commit message of your change. While zuul is building working directories to run jobs in, it will see this dependency, and use the branch/PR/etc. that you have submitted as its basis for pre-merge testing. And when all reviewers are happy with your dependent change, it won't land until the upstream dependency lands too.

So now you, can build an entire future without having landed risky code in master, and without having to wait for your upstream dependencies. This even works if the upstream project doesn't use Zuul. As long as you can give Zuul credentials to inspect the change management system and pull the necessary commits, it will be able to build a speculative future from them, and it won't accidentally land your dependent change until the upstream change is merged.

This even works across Gerrit, Github, and Github Enterprise. Other systems are also in development. This is nice if, say, you have GHE internally, and your depend on upstream Github projects, you can encourage your engineers to submit code upstream. And if you really want to get involved deeply with that upstream, you can even set up your zuul as a GitHub app, and report statuses on those repositories, which should help them avoid merging code that breaks you.

Now that you've embraced Zuul's future-building capabilities, you'll want to start expanding coverage. Zuul has all of what you'd probably expect for doing the easy stuff: linters, unit tests, etc. But what about full integration tests like our "build a mini-cloud" job we talked about before?

For this, Zuul is going to need a cloud. Current releases only support OpenStack clouds, or static pools of SSH/PowerShell accessible machines. However, there are several public cloud drivers, such as AWS and GCE, that are in various stages of quality. The AWS driver in particular is close to being ready.

Once you have given Zuul cloud resources via its sub-component named `nodepool`, you can start attaching nodesets to your jobs. These can be backed by various flavors, images, and configuration details, and given names and Ansible groups for use in playbooks. Typically youll even have a default nodeset in your base job that everything parents to.

At GoDaddy we have allocated 75 "large" instances (8GB RAM, 120GB of disk) on one of our less busy private OpenStack clouds for running tests. We also have defined a custom image using `nodepool-builder`, which gets built every 12 hours to pull in the latest apt and pip packages that our jobs will need. That way our job runtimes don't get too long with downloading and extracting new copies of things. We also ask `nodepool` to keep 15 nodes running at all times, so that any 5-node job will be able to start *immediately* and not have to wait for the cloud to spin up VMs. This also tends to smooth out problems with cloud control planes, which we do experience from time to time.

Alright, so now we have compute resources, opitimized images, and git repos plugged in to Zuul. What's next? We need to define jobs.

At GoDaddy we have some repos that have just 1 `noop` job on them. This still has a benefit, as the repo may be housing Zuul configuration that needs to be validated before changes are landed. We also have our most busy repo, which we call `openstack-deploy`, which runs between 3 and 9 jobs on every PR, and 3-4 jobs in the gate. The number varies, because sometimes we use Zuul's ability to skip jobs by filename. So, for instance, we don't need to run the big long `kolla-ansible` job which deploys a mini-cloud, if the change wholly consists of configuration details for our production clouds.

One really interesting aspect of working with Zuul is when you start to have interdependencies in repos. We have a repo which houses patches which we apply to the upstream OpenStack deployment tool named `kolla-ansible`. Whenever we run any deployments of kolla-ansible, we apply these patches on top, and generate the configuration for `kolla-ansible` on top of that. This means that if we update the patch repo with something that won't deploy right, we could end up with a broken master again.

But luckily, Zuul was built with this scenario in mind, and as such, allows us to have two repos run the same job in a shared queue. That means that if I propose a patch to `openstack-patches`, and an update to `openstack-deploy` that seems unrelated, they'll be tested together, one landing before the next using Zuul's speculative execution capabilities. If we do this right, it means we can't land broken code in either repo.

Finally, for those times where you just can't figure out why a job is failing, there's the "auto hold". Zuul can hold on to test nodes after a job fails if you inform it that you need it to. This allows an engineer to log in to the test nodes and poke around, even to try running the test again with modified code. Many of our biggest refactors happened on held test nodes, where an engineer would fiddle with things on those VMs, and then pull the changes back down and submit a fixed patch over the course of a few days.

So, from a cultural perspective, how did having these new capabilities affect team productivity?

First and foremost, there's a good chance if we didn't have Zuul, we would have crumbled under the pressure of a very tight deadline. With many many changes in flight and moving rapidly, it would have been an absolute momentum killer to have to stop and fix broken builds every day. Furthermore, by being able to let Zuul spin up mini-clouds, we gave our developers a 'fire and forget' mechanism for testing their changes in parallel, isolated from each other while the changes were in chaos.

Second, we found that members of the team were able to find more information about how things work faster, because everything, including the configuration for the actual tests, is stored in git trees. It really does help when you're doing a `git annotate` on a file, and then the change for a line that is confusing you is accompanied by edits to the testing configuration. This is especially helpful in root-cause analysis, where you are trying to match timelines from multiple sources together to find when a change may have been made that resulted in an incident.

Finally, Zuul was actually only half of the story. Another big part of it was that because Zuul was just running Ansible, we were able to leverage that for other tasks. We don't actually run our Ansible against production using Zuul. Instead, we run the same playbook that Zuul does with our super duper chat bot named Padre. I very much hope that we can open source Padre soon, and present it at a future AnsibleFest. But ultimately, Zuul was relatively straight forward to adopt because we could use the tool we knew already: Ansible, and it was also easy to break back out of Zuul when we needed to, for the same reason.

So what should you do if you are interested?

* Attend Ricardo Carillo Cruz's deep dive in to Zuul and Ansible Networking at 3:00pm.

* Come talk to us at the Zuul booth!

* Deploy zuul -- I'm not going to lie, this isn't easy, and it doesn't make a ton of sense unless you have your code in either GitHub, GitHub Enterprise, or Gerrit.
