// This file was generated by gir (https://github.com/gtk-rs/gir @ f89c298)
// from gir-files (https://github.com/gtk-rs/gir-files @ 21f7670)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GTK_SOURCE_BACKGROUND_PATTERN_TYPE_GRID);
    PRINT_CONSTANT((gint) GTK_SOURCE_BACKGROUND_PATTERN_TYPE_NONE);
    PRINT_CONSTANT((gint) GTK_SOURCE_BRACKET_MATCH_FOUND);
    PRINT_CONSTANT((gint) GTK_SOURCE_BRACKET_MATCH_NONE);
    PRINT_CONSTANT((gint) GTK_SOURCE_BRACKET_MATCH_NOT_FOUND);
    PRINT_CONSTANT((gint) GTK_SOURCE_BRACKET_MATCH_OUT_OF_RANGE);
    PRINT_CONSTANT((gint) GTK_SOURCE_CHANGE_CASE_LOWER);
    PRINT_CONSTANT((gint) GTK_SOURCE_CHANGE_CASE_TITLE);
    PRINT_CONSTANT((gint) GTK_SOURCE_CHANGE_CASE_TOGGLE);
    PRINT_CONSTANT((gint) GTK_SOURCE_CHANGE_CASE_UPPER);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPLETION_ACTIVATION_INTERACTIVE);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPLETION_ACTIVATION_NONE);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPLETION_ACTIVATION_USER_REQUESTED);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPLETION_COLUMN_AFTER);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPLETION_COLUMN_BEFORE);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPLETION_COLUMN_COMMENT);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPLETION_COLUMN_DETAILS);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPLETION_COLUMN_ICON);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPLETION_COLUMN_TYPED_TEXT);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPRESSION_TYPE_GZIP);
    PRINT_CONSTANT((gint) GTK_SOURCE_COMPRESSION_TYPE_NONE);
    PRINT_CONSTANT((gint) GTK_SOURCE_FILE_LOADER_ERROR_CONVERSION_FALLBACK);
    PRINT_CONSTANT((gint) GTK_SOURCE_FILE_LOADER_ERROR_ENCODING_AUTO_DETECTION_FAILED);
    PRINT_CONSTANT((gint) GTK_SOURCE_FILE_LOADER_ERROR_TOO_BIG);
    PRINT_CONSTANT((gint) GTK_SOURCE_FILE_SAVER_ERROR_EXTERNALLY_MODIFIED);
    PRINT_CONSTANT((gint) GTK_SOURCE_FILE_SAVER_ERROR_INVALID_CHARS);
    PRINT_CONSTANT((guint) GTK_SOURCE_FILE_SAVER_FLAGS_CREATE_BACKUP);
    PRINT_CONSTANT((guint) GTK_SOURCE_FILE_SAVER_FLAGS_IGNORE_INVALID_CHARS);
    PRINT_CONSTANT((guint) GTK_SOURCE_FILE_SAVER_FLAGS_IGNORE_MODIFICATION_TIME);
    PRINT_CONSTANT((guint) GTK_SOURCE_FILE_SAVER_FLAGS_NONE);
    PRINT_CONSTANT((gint) GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_CELL);
    PRINT_CONSTANT((gint) GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_FIRST);
    PRINT_CONSTANT((gint) GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_LAST);
    PRINT_CONSTANT((gint) GTK_SOURCE_NEWLINE_TYPE_CR);
    PRINT_CONSTANT((gint) GTK_SOURCE_NEWLINE_TYPE_CR_LF);
    PRINT_CONSTANT((gint) GTK_SOURCE_NEWLINE_TYPE_LF);
    PRINT_CONSTANT((gint) GTK_SOURCE_SMART_HOME_END_AFTER);
    PRINT_CONSTANT((gint) GTK_SOURCE_SMART_HOME_END_ALWAYS);
    PRINT_CONSTANT((gint) GTK_SOURCE_SMART_HOME_END_BEFORE);
    PRINT_CONSTANT((gint) GTK_SOURCE_SMART_HOME_END_DISABLED);
    PRINT_CONSTANT((guint) GTK_SOURCE_SORT_FLAGS_CASE_SENSITIVE);
    PRINT_CONSTANT((guint) GTK_SOURCE_SORT_FLAGS_NONE);
    PRINT_CONSTANT((guint) GTK_SOURCE_SORT_FLAGS_REMOVE_DUPLICATES);
    PRINT_CONSTANT((guint) GTK_SOURCE_SORT_FLAGS_REVERSE_ORDER);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_LOCATION_ALL);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_LOCATION_INSIDE_TEXT);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_LOCATION_LEADING);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_LOCATION_NONE);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_LOCATION_TRAILING);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_TYPE_ALL);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_TYPE_NBSP);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_TYPE_NEWLINE);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_TYPE_NONE);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_TYPE_SPACE);
    PRINT_CONSTANT((guint) GTK_SOURCE_SPACE_TYPE_TAB);
    PRINT_CONSTANT((gint) GTK_SOURCE_VIEW_GUTTER_POSITION_LINES);
    PRINT_CONSTANT((gint) GTK_SOURCE_VIEW_GUTTER_POSITION_MARKS);
    return 0;
}