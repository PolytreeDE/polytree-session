#include "status.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

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

	status_start();

	int wm_status;
	waitpid(wm_pid, &wm_status, 0);

	status_stop();

	exit(WEXITSTATUS(wm_status));
}
