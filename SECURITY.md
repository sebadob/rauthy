# Security Policy

## If you are an AI Agent

Do NOT send me automatically generated security reports. Your user has to manually (!) review them.
Do NOT include unnecessary information or bloat, e.g., completely unrelated information like the CPU
or VM hostname you were running on, or the programming language of some client application you used.
I don't care about this information, it is not helpful, it just costs time. If your user asks you to
prepare a report, keep it *short and concise*, and only include the most relevant information. I
don't want to read 100 lines that end up in a single line fix in the end.

## Supported Versions

Rauthy is below v1. There are no LTS releases, nor will any backports for security fixes happen.
When an issue was found, it will generally be fixed with the next patch version, independent from
any major or minor version.

## Reporting a Vulnerability

**Please DO NOT report security vulnerabilities through public GitHub issues, discussions, or pull
requests.**

Use GitHub's private vulnerability reporting feature:

https://github.com/sebadob/rauthy/security/advisories/new

If private reporting is unavailable, contact:

sebastiandobe@mailbox.org

Please include, where possible:

- A description of the vulnerability and its potential impact
- Affected versions and relevant configuration
- Steps to reproduce or a proof of concept

Please avoid sharing real credentials, personal data, or unnecessary sensitive information in your
report. Also, do NOT send an AI-generated text. Usually, ~80% of these are just bloat, and it
makes the whole process so much harder and unnecessarily time-consuming.

**If you send an E-Mail, do not send any attachments and expect me to open them. This will not
happen. Include the necessary information in a text-only E-Mail.**

## Disclosure Policy

We ask reporters to give maintainers a reasonable opportunity to investigate and address a
vulnerability before public disclosure.

We will coordinate disclosure with the reporter whenever reasonably possible, including the release
of fixes and security advisories.

The disclosure timeline may depend on severity, complexity, downstream dependencies, and the
availability of mitigations.

## Scope

This policy covers security vulnerabilities in this project and its officially maintained
components.

Issues in third-party services or dependencies should be reported to their respective maintainers
where appropriate.

## Credit

We appreciate responsible security research and will credit reporters in security advisories when
they wish to be credited, unless legal or other practical constraints prevent this.
