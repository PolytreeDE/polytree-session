#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#define WM_STARTUP_TIME_SEC 3

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

	for (unsigned int i = 0; i < WM_STARTUP_TIME_SEC; ++i) sleep(1);

	int wm_status;
	waitpid(wm_pid, &wm_status, 0);

	exit(WEXITSTATUS(wm_status));
}
