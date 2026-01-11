cmake_minimum_required(VERSION 3.25)

project(
  AOC2025
  VERSION 0.0.1
  DESCRIPTION "Solution to Advent of Code 2025 puzzles"
)

# Set a default build type if none was specified
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  message(STATUS "Setting build type to 'Release' as none was specified.")
  set(CMAKE_BUILD_TYPE Release CACHE STRING "Choose the type of build." FORCE)
  # Set the possible values of build type for cmake-gui
  set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS "Debug" "Release"
    "MinSizeRel" "RelWithDebInfo")
endif()

set(CMAKE_COLOR_DIAGNOSTICS ON)

file(GLOB CHILD_DIRS RELATIVE "${CMAKE_SOURCE_DIR}" "${CMAKE_SOURCE_DIR}/*")

foreach (subdir ${CHILD_DIRS})
  if (IS_DIRECTORY "${CMAKE_SOURCE_DIR}/${subdir}")
    file(GLOB SRC_FILES "${CMAKE_SOURCE_DIR}/${subdir}/*.cpp")
    if (SRC_FILES)
      # Create an executable named after the directory
      add_executable(${subdir} ${SRC_FILES})
      target_compile_features(${subdir} PRIVATE cxx_std_23)
      set_target_properties(${subdir} PROPERTIES
        CXX_STANDARD_REQUIRED ON
        CXX_EXTENSIONS OFF
      )
      target_compile_options(${subdir} PRIVATE
        $<$<CXX_COMPILER_ID:GNU,Clang,AppleClang>:-Wall -Wextra -Wfloat-conversion -pedantic -pedantic-errors>
        $<$<AND:$<CONFIG:Release,MinSizeRel,RelWithDebInfo>,$<CXX_COMPILER_ID:GNU,Clang,AppleClang>>:-march=native -ffast-math>
      )
    endif()
  endif()
endforeach ()
