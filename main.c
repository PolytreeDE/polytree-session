#include <pthread.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#include <X11/Xlib.h>

static bool datetime_running = false;
static Display *datetime_display = NULL;
static pthread_t datetime_thread;
static bool datetime_thread_created = false;

static bool datetime_start();
static void datetime_stop();
static void *datetime_run(void *arg);

int main()
{
	const pid_t wm_pid = fork();

	if (wm_pid == -1) {
		perror("polytree-session: WM fork");
		exit(EXIT_FAILURE);
	}

	if (wm_pid == 0) {
		char *const args[] = { "polytreewm", NULL };
		execvp(args[0], args);
		perror("polytree-session: WM exec");
		exit(EXIT_FAILURE);
	}

	datetime_start();

	int wm_status;
	waitpid(wm_pid, &wm_status, 0);

	datetime_stop();

	exit(WEXITSTATUS(wm_status));
}

bool datetime_start()
{
	// TODO: maybe we should assert
	if (datetime_running) return false;
	datetime_running = true;

	if ((datetime_display = XOpenDisplay(NULL)) == NULL) {
		datetime_stop();
		return false;
	}

	if (!(datetime_thread_created =
			pthread_create(&datetime_thread, NULL, datetime_run, NULL) == 0))
	{
		datetime_stop();
		return false;
	}

	return true;
}

void datetime_stop()
{
	// TODO: maybe we should assert
	if (!datetime_running) return;
	datetime_running = false;

	if (datetime_thread_created) pthread_join(datetime_thread, NULL);
	if (datetime_display) XCloseDisplay(datetime_display);
}

void *datetime_run(__attribute__((unused)) void *const arg)
{
	while (datetime_running) {
		time_t curr_time;
		time(&curr_time);

		const struct tm *const curr_tm = localtime(&curr_time);

		char buffer[256];
		strftime(buffer, sizeof(buffer), "%a, %e %b %Y, %H:%M:%S", curr_tm);

		XStoreName(datetime_display,
					DefaultRootWindow(datetime_display),
					buffer);
		XSync(datetime_display, False);

		sleep(1);
	}

	return NULL;
}
