# 1. Mål og rammer

## 1.1 Bakgrunn

*See [project-description.md](./project-description.md)*

## 1.2 Prosjektmål

Develop a proof-of-concept SSI application, and use the proof-of-concept to discover and present strengths and weaknesses of the current SSI infrastructure, with a focus on interoperability. 

There are two key areas of interoperability:
1. Interoperability between different did-methods - How and where DIDs are stored.
2. Interoperability between different wallets/agents - Where ownership of DIDs are prooved and where VCs are issued, stored, communicated and verified.

## 1.3 Rammer

The BSc project runs between `01.feb.2021` and `20.may.2021`.

The BSc project will be developed as an open source project on the DIN-Foundation's Github page here: https://github.com/DIN-Foundation/bsc-ntnu-2020/. 

See https://www.din.foundation/ for more info about the DIN-foundation.


# 2. Omfang

## 2.1 Fagområde

The project is developed within the field of Self-sovereign-identity - SSI.

Here are some links to get started:
- DIDs (Decentralized identifiers) by W3C: https://www.w3.org/TR/did-core/
- VC (Verifiable credentials) by W3C: https://www.w3.org/TR/vc-data-model/
- Working standards under DIF working groups: https://identity.foundation/#wgs

Also see [project-description.md](./project-description.md#Background) for more info.

## 2.2 Avgrensing

The project is looking at interoperability of the current SSI-infrastructure. More specifically interoperability between wallets, and interoperability between DID-methods.

To understand interoperability a proof-of-concept application will be developed. The proof-of-concept will implement one specific use-case of SSI. 

The proof-of-concept should implement the following parts of an SSI-workflow:
- Create an issuing-agent linked to a DID.
- Create a holder-agent linked to a DID.
- Create a verifying-agent linked to a DID.
- Issue a credential to a holder.
- Enable holder to present credential to a verifier.
- Enable verifier to verify credential presented by holder.

A limited set of ledgers and wallets should be investigated:

**@TODO Make list of wallets to investigate**

**@TODO Make list of ledgers to investigate**

## 2.3 Oppgavebeskrivelse

*See [project-description.md](./project-description.md)*

### User story

*Disclaimer: The BSc has no affiliation with Statens Vegvesen at the moment of writing, and the story below is pure fantasy.*

The norwegian driver license issuer, Statens Vegvesen, is considering to start issuing it's driver licenses as verifiable credentials (VCs). Statens Vegvesen is not sure if verifiable credentials is the future yet, but are willing to try and dip it's toes in the water. Statens vegvesen will still issue credentials in the traditional way for the forseeable future.

What Statens Veivesen want is a proof-of-concept SSI-application which will issue, hold and verify driver-licenses. They are hoping that this will bootstrap SSI in Norway, as this will enable other individuals and organizations to start experimenting with holding and verifying a serious credential which actually is useful.

The proof-of-concept may demonstrate that a driver license as a verifiable credential could be considered on par, legally speaking, with traditional driver licenses.

Statens Vegvesen wants the application to follow open standards which will enable the application to be agnostic about where and how credentials are issued, stored and verified. In other words Statens Vegvesen want to avoid the application to be locked to a specific ledger and a specific wallet.


### Deliverables

- High-level design document.
- Source code of a proof-of-concept SSI application.
- Deployment of a proof-of-concept SSI application.
- A comparison matrix of ledger interoperability.
- A comparison matrix of wallet interoperability.
- Bachelor project report.

# 3. Prosjektorganisering

## 3.1 Ansvarsforhold og roller

| Navn        | Role                                                            |
|-------------|-----------------------------------------------------------------|
| Jonas       | Developer                  (Service Delivery Manager in Kanban) |
| Snorre      | Client and domain expert 1 (Service Request Manager in Kanban)  |
| Mariusz     | Domain expert 2                                                 |
| Abylay      | Domain expert 3                                                 |
| Deepti      | Academic supervisor                                             |

## 3.2 Rutiner og regler i gruppa

### Developer rules

- Rule 1: To keep the project moving at all times, work on the project every day of the week. Minimum 5 minutes, maximum 8 hours.
- Rule 2: Average 30 hours each week.
- Rule 3: Respond to NTNU-mail, Discord-messages, Github-comments, Phone-calls, SMS in less than 24 hours.
- Rule 4: Write meeting-notes for every meeting.
- Rule 5: Log all hours in Toggl under these 5 labels: [Meeting, Organizing, Writing, Coding, Researching]. These labels is not based on any standard, and is just something the author has found useful based on experience.
- Rule 6: All documents relevant to the project, should be found in the Github repository, unless it contains sensitive/private information.

# 4. Planlegging, oppfølging og rapporter

## 4.1 Hovedinndeling av prosjektet

The project is loosely driven forward by the Kanban framework. To keep track of progress in a Kanban project the Kanban board is essential. See the board here: https://github.com/DIN-Foundation/bsc-ntnu-2020/projects/1.

Our Kanban-board has 5 columns:

**1. Backlog**

List of tasks which have been thought of, but is lacking details like what needs to be done and when the implementation is supposed to happen.

Tasks in backlog are created whenever someone feels like it. There are no requirements to put something in the backlog. The treshold should be as low as possible. Eqaully, the treshold for deleting a backlog-task should be equally low.

**2. Todo**

List of tasks that are planned to be implemented in the near future, and have enough details ironed out to make it possible to start the implementation.

Tasks are moved from `Backlog` to `Todo`:
- After being discussed in the weekly domain-experts meeting.
- **or** after an ad-hoc meeting dedicated to discussing a specific task.
- **or** after the developer has researched/learned something which makes a task "obviously implementable". If a developer does this, it should be clearly communicated in the next weekly domain-experts meeting.

**3. In Progress**

List of tasks that are in progress. A developer have written words or code. These tasks should ideally be linked to an open Pull Request on Github with a WIP label on it.

**4. In Review**

List of tasks where the developer has implemented the task, and is waiting to get a stamp of approval from a second pair of eyes. Approval could be given by the client, supervisor or any of the domain experts. The developer will request review from the specific person which is considered best suited to review the pull request. This specific person will be notified about this on Discord, and via email if necessary.

**5. Done**

List of task where the pull request linked to the task has been approved and merged into the main branch of the git source tree.


### Why Kanban instead of Scrum?

Most Scrum projects implements a Kanban-board, and I would argue that Scrum could be considered a superset of Kanban, but they are really orthogonal concepts. It is not either or. You could have a Kanban-only, Kanban+Scrum or Scrum-only project.

Since the developer-team consist of only 1 person, Scrum is considered too much overhead, since all the Scrum rituals has to be organized and executed by a single person. Kanban is lower overhead, by requiring fewer meetings, fewer roles, fewer rituals, making Kanban more flexible but also less structured than Scrum.

Scrum has a focus on delivering something every sprint. Kanban does not say anything about how often something should be delivered. Since this project is research-heavy, there is still a big question-mark about what should be made, and when something should be made. Of course it would be possible to schedule a demo every 2 weeks to show... something. The feeling here is that we need more flexibility than that. 

A demo will be held on the weekly meetings showcasing what has been done the last week, if something worth showing has been done. This would be similar to having 1 week sprints in Scrum. Maybe we are having Kanban with 1 week sprints?

Scrum is very strict about when new tasks should be scheduled - Sprint planning meeting - but we want more flexibility than that. We don't want a dedicated meeting only for planning.

Scrum is very strict about when completed tasks should be demoed - Sprint review meeting - but we want more flexibility. We don't want a dedicated Sprint review meeting.

Scrum splits it's process into sprints of equal length, with a defined start, and end, and duration. We want more flexibility about when something starts and when something ends, and endures.

Scrum requires a daily meeting - Daily Scrum. Since we are all working remotely, and everyone but the single developer, has other projects as their main focus, requiring a daily meeting for this project, is considered impractical.

The whole Kanban vs Scrum discussion, in our case, could be boiled down to flexibility vs structure. Given the current state of this project, we are valuing flexibility over structure. This could change as the project evolves, which leaves the door open for Scrum to do a comeback down the line. 

*See: [The 4 Scrum Ceremonies Made Simple: A Quick Guide To Scrum Meetings](https://medium.com/ideas-by-crema/the-4-scrum-ceremonies-made-simple-a-quick-guide-to-scrum-meetings-ea91fe604d88)*

## 4.2 Plan for statusmøter og beslutningspunkter i perioden

We have planned 2 weekly meetings, 1 with supervisor and 1 with client and domain experts.

**Agenda for domain experts meeting: Tuesdays @ 12:30**

Walk the Kanban-board backwards, from left to right:

- Present completed tasks the last week, if any.
- Discuss in-review tasks.
- Discuss in-progress tasks.
- Discuss todo tasks.
- Discuss backlog, if there is any time left.

If we are short on time, focus on the most important tasks first.

**Academic supervisor meeting: Tuesdays @ 13:00**

- Discuss updates to execution plan, if any, and explain why the update was necessary.
- Discuss the progress on the report. 

Participants of these meetings have received invites by email, to make it easy to add the scheduled meetings to their respective calendars.

# 5. Organisering av kvalitetssikring

## 5.1 Dokumentasjon, standardbruk og kildekode

The language used to develop the proof-of-concept app as not been decided yet, but we would like to use a language with mature tools for documentation, testing, linting, formatting. These tools should be run by the CI on every commit to the source-tree to ensure consistent quality.

Here are some key areas of quality control, which can be expected:

**API documentation generated from code docstrings**

This is a very useful tool to document interfaces in the code bases. Either interfaces between different modules in the code-base, or external interfaces like a CLI or HTTP API etc.

**High level visual documentation**

Some high-level documentation will be required to explain the architecture of the entire solution, using diagrams.

**Testing of API's**

There will be an interface to control the proof-of-concept app. Most of the testing we do should focus heavily on testing this interface, be it a CLI or a HTTP API or both or something else. How this interface will look is TBD.

**Linting and formatting using community defaults and best practices**

Programming communities have standards and default practices regarding how to write code. Automated tools like a linter should run in the CI environment to ensure that these best practices are being followed.


## 5.2 Konfigurasjonsstyring

### OS and platform support

The proof-of-concept application will most likely be a mix between an CLI and a Web-app - TBD. The details of the app architecture is yet to be figured out. As a general principle, we should try to be cross-browser and cross-shell, which will in turn make us cross-OS. This is in the spirit of the project, as we are trying to measure interoperability in the SSI-space. Practice as you preach.

Shells to support:
- [zsh](https://en.wikipedia.org/wiki/Z_shell) - Default shell on MacOS
- [bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) - Default shell on most Linux distributions

Browsers to support:
- Chromium (Chrome, IE, Opera, Brave, etc.) - Most used browser technology in the world
- Firefox - Default browser on most Linux distributions
- Safari - Default browser on MacOS

### Collaboration Tools

A list of tools required participate in the project:

- [Git](https://git-scm.com/) - Version control
- [Github](https://github.com/) - Host files, issues, pull-requests
- [Discord](https://discord.com/) - Direct messaging
- [Google Meet](https://meet.google.com/) - For domain experts meeting
- [MS Teams](https://www.microsoft.com/en/microsoft-teams/group-chat-software) - For academic supervisor meeting

### [TBD] Development environment, programming languages, frameworks

The initial phase of the project will be spent trying to figure out this part, together with the overall architecture of the application. A general principle here is that we want to build on top of an existing codebase. We do not want to start from scratch.

- Hyperledger Aries has a nice overview over different frameworks that can be used to develop SSI-agents in different languages. See: https://github.com/hyperledger/aries#aries-agent-frameworks.
- Trinsic also mentions `Sidetree`, `Universal Resolver` and `KERI` in their overview, under the DIF codebases section. See: https://trinsic.id/open-source-ssi-codebases/.


## 5.3 Risikoanalyse (identifisere, analysere, tiltak, oppfølging)

### Risk 1: Proof-of-concept app development-cost unpredictability

There is a big risk that the time it takes to develop the proof-of-concept app will be very unpredictable. Most of the development happens in an area of programming which is completely new to the developer of the app. This makes the learning curve steep, and the time-estimate more upredictable. Hopefully the developer will draw from his experience of working in other fields, and adapt quickly, but it is hard to know exactly how much time will be spent to adapt to the new development environment.

**Risk mitigation strategy**
The weekly domain-experts meeting is an important arena to show progress, and to discover delays in progress. In this meeting we will adjust our strategy to try and stick to the schedule. There should be a dedicated agenda point in this meeting to address this risk during the implementaiton-phase of the app.

# 6. Plan for gjennomføring

The project starts in Week 5 (01.feb - 07.feb) and ends in Week 20 (17.may - 23.may). The deadline for delivering the project is 20.may.

### Timespan
- 15 weeks and 3 days
- 108 days
- 78 weekdays
- 624 weekdayhours (8 hours per weekday)

### Milestones Gantt chart

| Milestones                          |Week 5|Week 6|Week 7|Week 8|Week 9|Week 10|Week 11|Week 12|Week 13|Week 14|Week 15|Week 16|Week 17|Week 18|Week 19|Week 20|
|-------------------------------------|------|------|------|------|------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|
| High-level design doc               |  x   |  x   |      |      |      |       |       |       |       |       |       |       |       |       |       |       |
| Learn devenv                        |      |  x   |  x   |      |      |       |       |       |       |       |       |       |       |       |       |       |
| Proof of concept app implementation |      |      |  x   |   x  |  x   |   x   |   x   |   x   |   x   |   x   |       |       |       |       |       |       |
| DID-method interop investigation    |      |      |      |      |      |       |       |   x   |   x   |   x   |   x   |       |       |       |       |       |
| Wallet interop investigation        |      |      |      |      |      |       |       |       |   x   |   x   |   x   |   x   |       |       |       |       |
| Research SSI                        |  x   |  x   |  x   |   x  |  x   |   x   |   x   |   x   |   x   |   x   |   x   |   x   |   x   |       |       |       |
| Writing Bachelor Report                     |  x   |  x   |  x   |   x  |  x   |   x   |   x   |   x   |   x   |   x   |   x   |   x   |   x   |  x    |   x   |   x   |


### Milestones description

**High-level design doc (1-2 week)**

A document which will lay out the design/architecture of the proof-of-concept application. This high-level design document will be used to kick-start the implementation. The implementation is expected to iterate on the high-level design, and if there are significant changes, this document needs to be kept up to date throughout the process. The document should be checked into git, and available on Github.

**Learn devenv (1-2 weeks)**

Once the initial high-level design has been completed, it is time to get to know the frameworks, languages, tools. It is time to get familiar with the development environment. Of course this is a process that is expected to continue throughout the project, but 1 week has been scheduled dedicated to this milestone.

**Proof-of-concept app implementation (9 weeks)**

This milestone is when the coding will start to happen. Time to get our hands dirty. Common tasks expected here, is coding, testing, updating documentation, deploying. There will be frequent iteration, testing out ideas in practice, and quickly turn around and go in different directions, if required. This phase is expected to take up most of the time of the project spanning 9 weeks.

**DID-method interop investigation (4 weeks)**

To contribute to the community, we want to complete an investigation into how the proof-of-concept application will work across multiple DID-methods. During this period of 4 weeks, some sort of overview in the form of a matrix comparing different methods will be produced.

**DID-agent interop investigation (4 weeks)**

The same milestone as above, but investigating different agents/wallets from different vendors. TODO: Figure out if there is some overlap between this milestone and the one above, and if they can be merged to a single milestone.

**Research SSI (13 weeks)**

Expected to be something which will be done most of the project, but should be kept to a minimal the last 2 weeks of the project, to leave room to focus on writing the bachelor report. Research includes reading papers, watching vidoes, crawling Github projects, reading w3c-specifications, talking to community members, etc.

**Writing bachelor report (15 weeks)**

This should be the first thing we start doing, and the last thing we stop doing. Always keep an eye on the progress of the bachelor report. The 3 last weeks of the bachelor project is left exclusively to work on this milestone.