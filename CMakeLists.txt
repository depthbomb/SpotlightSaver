cmake_minimum_required(VERSION 4.0)

project(SpotlightSaver VERSION 6.0.0 DESCRIPTION "Windows Spotlight Saver" LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 26)

set(COMPANY_NAME "Caprine Logic")
set(PROJECT_COPYRIGHT "(c) 2022-2025 ${COMPANY_NAME}")

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_SOURCE_DIR}/bin/Debug)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_SOURCE_DIR}/bin/Release)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELWITHDEBINFO ${CMAKE_SOURCE_DIR}/bin/RelWithDebInfo)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_MINSIZEREL ${CMAKE_SOURCE_DIR}/bin/MinSizeRel)

set(APP_RC "${CMAKE_SOURCE_DIR}/spotlight_saver.rc")

configure_file(
        ${CMAKE_SOURCE_DIR}/cmake/spotlight_saver.rc.in
        ${CMAKE_SOURCE_DIR}/spotlight_saver.rc
        @ONLY
)

configure_file(
        ${CMAKE_SOURCE_DIR}/cmake/setup.iss.in
        ${CMAKE_SOURCE_DIR}/setup.iss
        @ONLY
)

add_executable(${PROJECT_NAME} src/main.cc ${APP_RC})

target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/src )

if (CMAKE_BUILD_TYPE MATCHES "MinSizeRel|Release")
    set_target_properties(${PROJECT_NAME} PROPERTIES
            INTERPROCEDURAL_OPTIMIZATION TRUE
    )

    if(MSVC)
        set_property(TARGET ${PROJECT_NAME} PROPERTY
                MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
    endif()
endif ()
