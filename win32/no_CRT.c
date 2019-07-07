#define WIN32_LEAN_AND_MEAN  // Exclude rarely-used 
#define STRICT
#include <windows.h>

void mainCRTStartup(void)
{
	ExitProcess(99);
}