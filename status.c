#include "status.h"

#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

static bool running = false;
static pid_t pid = 0;

bool status_start()
{
	// TODO: maybe we should assert
	if (running) return false;
	running = true;

	pid = fork();

	if (pid == -1) {
		status_stop();
		return false;
	}

	if (pid == 0) {
		char *const args[] = { "slstatus", NULL };
		execvp(args[0], args);
		perror("polytree-session: slstatus exec");
		exit(EXIT_FAILURE);
	}

	return true;
}

void status_stop()
{
	// TODO: maybe we should assert
	if (!running) return;
	running = false;

	kill(pid, SIGKILL);
	waitpid(pid, NULL, 0);
}
