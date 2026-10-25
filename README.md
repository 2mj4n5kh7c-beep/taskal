# Taskal

Course tasks and a weekly lecture tracker for university students.

- **Tasks** sync from your university's course calendar (Moodle, Canvas and anything else that publishes an iCal link). New assignments appear and changed deadlines update on their own.
- **Lectures**: your weekly timetable for every week of the semester. Mark each session as seen, live or later, and add notes like "seen till 00:57".
- **Semesters** with holiday presets for Israeli universities, so holidays show as "no class".

## Get it

- **Windows / Mac app** (automatic calendar sync): [latest release](https://github.com/2mj4n5kh7c-beep/taskal/releases/latest)
- **Web version** (try it in a browser, import calendar files by hand): https://2mj4n5kh7c-beep.github.io/taskal/

Your data stays on your own device.

## Connecting your calendar

In Moodle: **Calendar → Import or export calendars → Export calendar → All courses, Recent and next 60 days → Get calendar URL**. Paste that link into Taskal. Many universities open a new Moodle site every year, so you may need a new link each October.

## For developers

The app is a single page in `app/index.html`. `src-tauri/` wraps it as a desktop app (Tauri 2) and adds a native `fetch_calendar` command so calendar links can be read without browser CORS limits.

- Run locally: `npm install && npx tauri dev`
- Release: bump the version in `src-tauri/tauri.conf.json`, then push a tag `vX.Y.Z`. GitHub Actions builds the Windows installer and the Mac app.
