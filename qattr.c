/*
 * qattr.c
 * Program file for the attribute module.
 */

#include <stdlib.h>
#include <assert.h>
#include <stdint.h>

#include "qdefs.h"

#include "qattr.h"

QattrList_t*
qattr_list_create(size_t count){
	assert(count < SIZE_MAX);
	QattrList_t  qattr_dummy;
	QattrList_t *qattr_listp;
	qattr_listp = &qattr_dummy;
	qattr_listp->attrp = calloc(count, sizeof(*(qattr_listp->attrp)));
	assert(qattr_listp->attrp != NULL);
	qattr_listp->count = count;
	return qattr_listp;
}

int
qattr_list_destroy(QattrList_t *qattr_list) {
	free(qattr_list);
}

int
QDatameta_t* qattr_list_value_get(QattrList_t*, QattrKey_t) {
	
}

int
qattr_list_set(QattrList_t *attr_list, QattrKey_t attr_key, QDatameta_t *datameta) {
	attr_list->key = attr_key;
	attr_list->va
}
