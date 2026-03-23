add_executable(conformance_test conformance_test.cpp)
target_link_libraries(conformance_test PRIVATE fast_agent_ingest)

add_test(NAME conformance COMMAND conformance_test
         WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/../../..")
