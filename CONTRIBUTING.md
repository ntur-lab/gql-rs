# NTUR LAB Contribution Guideline

First off, thank you for the time you'll eventually take to contribute! 🧡

<a id="code-of-conduct"></a>
## Code of Conduct

This project adheres to the Contributor Covenant [code of conduct](CODE-OF-CONDUCT.md).
By participating, you are expected to uphold this code.
Please report unacceptable behavior to [code-of-conduct@ntur-lab.com](mailto:code-of-conduct@ntur-lab.com])

Our community is based on mutual respect and consideration. We are using English as the common language for communication.
If English is not your native language, don't be afraid to ask for help in case you have difficulties expressing yourself.

We are establishing these rules to communicate effectively:

* Be respectful and considerate.
* When reaching out to us, the more effort you put into describing your concern, the easier it will be for us to review it and the more likely we are able to do so quickly.
* Keep your description focused and avoid excessive summarization or bloated explanations so that we can understand the essence of your request without working through walls of text.

<a id="using-github-issues"></a>
## Using GitHub Issues

We use GitHub issues to track bugs and enhancements.
If you have a general usage question, please ask on project specific [GitHub Discussions](https://docs.github.com/en/discussions/quickstart).

If you are reporting a bug, please help to speed up problem diagnosis by providing as much information as possible.
For non-trivial bugs provide a [Minimal Reproducable Example](https://stackoverflow.com/help/minimal-reproducible-example), preferably as a GitHub repository. Make sure to include the database, either as an in memory database or if that is not possible using https://www.testcontainers.org/[Testcontainers].
The preferred build tool is Cargo and our preferred language for bug reports is Rust (but of course it depends on the project).

<a id="security-vulnerabilities"></a>
## Reporting Security Vulnerabilities

If you think you have found a security vulnerability in any of the Ntur-Lab projects, please *DO NOT* disclose it publicly until we've had a chance to fix it.
Please don't report security vulnerabilities using GitHub issues, instead head over to https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/report-privately and learn how to disclose them responsibly.

<a id="how-to-contribute"></a>
## How to Contribute

<a id="discuss"></a>
### Discuss

If you have a question, check Github Discussion.
Find an existing discussion, or start a new one if necessary.

If you believe there is an issue, search through existing issues trying a few different ways to find discussions, past or current, that are related to the issue.
Reading those discussions helps you to learn about the issue, and helps us to make a decision.

<a id="quickstart"></a>
### Quickstart

For the impatient, if you want to submit a quick pull request (polishing, trivial fix, documentation fix):

1. API changes require discussion, use cases, etc.
   Code comes later.
   Create an issue in the GitHub issue tracker and stop here.
   In every other case continue with step 2.
2. Make sure you use the code formatters provided (`rustfmt.toml` or `.rustfmt.toml` already available in each rust project) and have them applied to your changes. 
3. Don't submit any formatting-related changes.
4. Make sure you submit test cases (unit or integration tests) that back your changes.
5. Try to reuse existing test sample code (domain classes).
   Try not to amend existing test cases but create new ones dedicated to the changes you're making to the codebase.
   Try to test as locally as possible but potentially also add integration tests.
6. In the Javadoc for the newly added test cases refer to the issue the changes apply to like this

    ```rust
    #[test]
    fn your_test_method() { ... }
    ```

7. Make sure you added yourself as author in the headers of the classes you touched.
   Amend the date range in the Apache License header if needed.
   For new types, add the license header (copy from another file and set the current year only).
8. Make sure you follow the <<commit-messages>> guidelines.
9. Make sure you provide your full name and an email address registered with your GitHub account.
10. You may sign your commits with a GPG key.
   As we generally cherry-pick commits for merging, it can happen that your signature is replaced by a signature from a maintainer that pushes your changes to the target branches.

<a id="participate-in-reviews"></a>
### Participate in Reviews

Helping to review pull requests is another great way to contribute.
Your feedback can help to shape the implementation of new features.
When reviewing pull requests, however, please refrain from approving or rejecting a PR unless you are a core committer for Spring Data.

<a id="reference-documentation"></a>
### Reference Documentation

The reference documentation lives within the code located at the `docs/` directory, in [Markdown format](https://www.markdownguide.org/).
For trivial changes, you may be able to browse, edit source files, and submit directly from GitHub.

When making changes locally, run `cargo doc --open` and then check the result in your browser.

<a id="commit-messages"></a>
### Commit messages

The commit message should follow the following style:

* The first line contains the summary starting with a capital letter.
* Finish the summary with a dot.
* In the description, don't use single line breaks.
  No manual wrapping.
  Separate paragraphs by a blank line.
* During merges, add a reference to the original pull request.
* Add related references at the very bottom (also see the section on pull requests below).

  > Summary of the commit (try to stay under 80 characters).
  > 
  > Additional explanations if necessary.
  > 
  > Original pull request #?? (can be omitted when there's no ticket using Closes #?? syntax)  
  > Closes #?? (optionally close tickets)   
  > Related tickets #??? (optionally refer to related tickets)

<a id="advanced"></a>
## Advanced

This section contains some advanced information, mainly targeted at developers of the individual projects.

<a id="advanced.general"></a>
### General

* Fix bugs in `main` first, if it's reasonable to port the fix back into a bugfix branch, try to do so with cherry-picking.
* Try to keep the lifespan of a feature branch as short as possible.
  For simple bug fixes they should only be used for code review in pull requests.
* On longer running feature branches, don't pull changes that were made to `main` in the meantime.
  Instead, rebase the feature branch onto the current `main`, sorting out issues and making sure the branch will fast-forward merge eventually.

<a id="advanced.dependency-management"></a>
### Dependency management

* When upgrading dependencies of a project, make sure you apply the following rules:
  ** For service releases of a release train, only upgrades to bugfix versions are allowed.
  I.e., if a GA release of a release train included a dependency in version 2.6.2, you must only upgrade to 2.6.x versions.
  Upgrading to 2.7 is *not allowed*.
  ** During the milestone phase of a new release train, upgrade to the latest version of a dependency unless compatibility with a former version is required.
  Upgrades to new major versions are allowed, too, but consider ways to support multiple major versions for one release train to allow a smoother transition.

<a id="advanced.benchmarks"></a>
### Benchmarks

Currently, we aren't using a specific benchmark tool.

Feel free to use or propose anything.

<a id="advanced.change-tracking"></a>
### Change tracking

* Each change must be associated with either a GitHub issue or a pull request.
* When an issue is first created, it is flagged `waiting-for-triage` waiting for a team member to triage it.
  Once the issue has been reviewed, the team may ask for further information if needed, and based on the findings, the issue is either assigned a target milestone or is closed with a specific status.
  When a fix is ready, the issue is closed and may still be re-opened until the fix is released.
  After that the issue will typically no longer be reopened.
  In rare cases if the issue was not at all fixed, the issue may be re-opened.
  In most cases, however, any follow-up reports will need to be created as new issues with a fresh description.
* Make sure you don't commit without referring to a GitHub issue.
  If we have a rather general task to work on, create a GitHub issue for it and commit against that one.
* Try to resolve a GitHub issue in a single commit.
  I.e., don't have separate commits for the fix and the test cases.
  When polishing pull requests requires some more effort, have a separate commit to clearly document the polishing (and attribute the efforts to you).
* We usually use feature branches to work on GitHub issues and potentially let multiple people work on a feature.
* Follow the commit message style described in <<quickstart>>.
  Especially the summary line should adhere to the style documented there.
* After pushing fixes to the remote repository, mark the GitHub issues as resolved in and set the earliest milestone that ships the fix according to which branches you pushed to.
* Avoid merge commits as they just tend to make it hard to understand what comes from where.
  Using the GitHub issue number in the commit message will allow us to keep track of commits belonging together.

<a id="advanced.code-style"></a>
### Handling pull requests

* Be polite.
  It might be the first time someone contributes to an OpenSource project, so we should forgive violations to the contribution guidelines.
  Use some gut feeling to find out whether it makes sense to ask the reporter to fix stuff or just go ahead and add a polishing commit yourself.
* Before we merge stuff back into `main`, make sure you rebase the branch.
  We generally do not allow merge commits, so a merge should always be fast-forwarded.
  The GitHub issue number and the timestamps give enough tracking information already.
* The simplest way to merge back a pull request submitted by someone external is ``curl``ing the patch into `git am`.
  You can then polish it by either adding a commit or amending the provided commit.
  Make sure you keep the original author when amending.

  ```
  curl $PULL_REQUEST_URL.patch | git am --ignore-whitespace
  ```

* If you merge back a feature branch and multiple developers contributed to that, try to rearrange the commits and squash them into a single commit per developer.
  Combine the commit messages and edit them to make sense.
* Before pushing the changes to the remote repository, amend the commit(s) to be pushed and add a reference to the pull request to them.
  This will cause the pull request UI in GitHub to show and link those commits.

  ```
  ...
  Original pull request: #??
  ```

## Attribution

This contribution guideline is adapted from the Spring Data Contribution Guideline, permanently available at https://github.com/spring-projects/spring-data-build/blob/main/CONTRIBUTING.adoc  