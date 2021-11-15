#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main()
{
	char *const args[] = { "polytreewm", NULL };
	execvp(args[0], args);
	fprintf(stderr, "polytree-session: execvp %s", args[0]);
	perror(" failed");
	exit(EXIT_FAILURE);
}
