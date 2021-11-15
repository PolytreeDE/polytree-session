#include "datetime.h"

#include <pthread.h>
#include <unistd.h>
#include <X11/Xlib.h>

static bool running = false;
static Display *display = NULL;
static pthread_t thread;
static bool thread_created = false;

static void *run(void *arg);

bool datetime_start()
{
	// TODO: maybe we should assert
	if (running) return false;
	running = true;

	if ((display = XOpenDisplay(NULL)) == NULL) {
		datetime_stop();
		return false;
	}

	if (!(thread_created = pthread_create(&thread, NULL, run, NULL) == 0)) {
		datetime_stop();
		return false;
	}

	return true;
}

void datetime_stop()
{
	// TODO: maybe we should assert
	if (!running) return;
	running = false;

	if (thread_created) pthread_join(thread, NULL);
	if (display) XCloseDisplay(display);
}

void *run(__attribute__((unused)) void *const arg)
{
	while (running) {
		time_t curr_time;
		time(&curr_time);

		const struct tm *const curr_tm = localtime(&curr_time);

		char buffer[256];
		strftime(buffer, sizeof(buffer), "%a, %e %b %Y, %H:%M:%S", curr_tm);

		XStoreName(display, DefaultRootWindow(display), buffer);
		XSync(display, False);

		sleep(1);
	}

	return NULL;
}
