Feature: parse::video_from_url

  Scenario: Happy path youtube url was given
    Given I have a youtube URL string
    When I parse it via video_from_url
    Then the video object should have the parsed properties

  Scenario: URL with no asset id was given
    Given I have a youtube URL with no asset id
    When I parse it via video_from_url
    Then the video object should have no asset id

  Scenario: The url is not a known platform
    Given I have a neocities URL string
    When I parse it via video_from_url
    Then the video object should have no asset id and an unknown platform
