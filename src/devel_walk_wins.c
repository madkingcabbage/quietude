#include <stdlib.h>
#include <ncurses.h>

#include "qdefs.h"
#include "qerror.h"

#include "mode.h"
#include "qattr.h"
#include "qwalk.h"
#include "devel_walk.h"



#define DEVEL_WALK_AREA_INFO_WIN_COLS 80
#define DEVEL_WALK_AREA_INFO_WIN_BORDER_COLS (DEVEL_WALK_AREA_INFO_WIN_COLS + 2)



/*@external@*/
extern int /*@alt void@*/wborder(WINDOW *, chtype, chtype, chtype, chtype, chtype, chtype, chtype, chtype);



/**
 * Initialize @c WINDOW types for devel_walk.
 * @param[out] area_win: @c WINDOW to output the relevant #QwalkArea_t.
 * @param[out] area_border_win: @c WINDOW to draw @p area_win border.
 * @param[out] info_win: @c WINDOW to output #QwalkObj_t info.
 * @param[out] info_border_win: @c WINDOW to draw @p info_win border.
 * @return #Q_OK or #Q_ERROR.
 */
int
devel_walk_wins_init(WINDOW **area_winp, WINDOW **area_border_winp,
		WINDOW **info_winp, WINDOW **info_border_winp) {
	*area_border_winp = newwin(LINES, (COLS - DEVEL_WALK_AREA_INFO_WIN_BORDER_COLS), 0, 0);
	if (*area_border_winp == NULL) {
		Q_ERRORFOUND(QERROR_ERRORVAL);
		abort();
	}
	/* return val from box is ignored because it's guaranteed to return OK. */
	box(*area_border_winp, 0, 0);
	
	*area_winp = derwin(*area_border_winp, LINES - 2, (COLS - DEVEL_WALK_AREA_INFO_WIN_BORDER_COLS) - 2, 1, 1);
	if (*area_winp == NULL) {
		Q_ERRORFOUND(QERROR_ERRORVAL);
		abort();
	}
	
	*info_border_winp = newwin(LINES, DEVEL_WALK_AREA_INFO_WIN_BORDER_COLS + 2,
			0, (COLS - 1) - (DEVEL_WALK_AREA_INFO_WIN_BORDER_COLS + 2));
	if (*info_border_winp == NULL) {
		Q_ERRORFOUND(QERROR_ERRORVAL);
		abort();
	}
	box(*info_border_winp, 0, 0);
	
	*info_winp = derwin(*info_border_winp, LINES - 2, DEVEL_WALK_AREA_INFO_WIN_COLS,
			0, (COLS - 1) - DEVEL_WALK_AREA_INFO_WIN_COLS);
	if (*info_winp == NULL) {
		Q_ERRORFOUND(QERROR_ERRORVAL);
		abort();
	}

	return Q_OK;
}


/**
 * Close @c WINDOW types for devel_walk.
 * @param[out] area_win: @c WINDOW for #QwalkArea_t output.
 * @param[out] area_border_win: @c WINDOW for drawing @p area_win border.
 * @param[out] info_win: @c WINDOW for #QwalkObj_t info output.
 * @param[out] info_border_win: @c WINDOW for drawing @p info_win border.
 * @return #Q_OK or #Q_ERROR.
 */
int
devel_walk_wins_close(WINDOW **area_winp, WINDOW **area_border_winp,
		WINDOW **info_winp, WINDOW **info_border_winp) {
	
	int returnval = Q_OK;
	if (delwin(*area_winp) == ERR) {
		Q_ERRORFOUND(QERROR_ERRORVAL);
		returnval = Q_ERROR;
	}
	if (delwin(*area_border_winp) == ERR) {
		Q_ERRORFOUND(QERROR_ERRORVAL);
		returnval = Q_ERROR;
	}
	if (delwin(*info_winp) == ERR) {
		Q_ERRORFOUND(QERROR_ERRORVAL);
		returnval = Q_ERROR;
	}
	if (delwin(*info_border_winp) == ERR) {
		Q_ERRORFOUND(QERROR_ERRORVAL);
		returnval = Q_ERROR;
	}

	return returnval;
}