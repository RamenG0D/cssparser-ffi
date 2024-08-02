function(add_cssparser_as_dependency TARGET_NAME)
    message(NOTICE "Adding CSSParser as dependency for ${TARGET_NAME}. Make sure to add as a submodule for your project, then include the cmake file.")
    set(CSSPARSER_INCLUDE_DIRS ${CMAKE_CURRENT_SOURCE_DIR}/include CACHE INTERNAL "")

    if(${CMAKE_SYSTEM_NAME} STREQUAL "Windows")
        message(NOTICE "CSSParser: Building for Windows")
        set(CSSPARSER_LIBRARIES ${CSSPARSER_LIBRARIES} CACHE INTERNAL "${CMAKE_CURRENT_FUNCTION_LIST_DIR}/release/c_cssparser.lib")
        add_custom_command(TARGET ${TARGET_PROJECT}
            COMMAND "run.bat"
            WORKING_DIRECTORY
        )
    elseif(${CMAKE_SYSTEM_NAME} STREQUAL "Linux")
        message(NOTICE "CSSParser: Building for Linux")
        add_custom_command(TARGET ${TARGET_PROJECT}
        COMMAND "run.sh"
        WORKING_DIRECTORY
        )
        set(CSSPARSER_LIBRARIES ${CSSPARSER_LIBRARIES} CACHE INTERNAL "${CMAKE_CURRENT_FUNCTION_LIST_DIR}/release/c_cssparser.a")
    endif()
endfunction()