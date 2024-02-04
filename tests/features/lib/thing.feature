Feature: Parsing stuff with parse_platform

  Scenario: Happy path youtube url was given
    Given I have a proper youtube video URL
    When I parse it via parse_platform
    Then the resulting platform should be the Youtube variant of the enum

  Scenario: Happy path Rumble URL was given
    Given I have a proper rumble video URL
    When I parse it via parse_platform
    Then the resulting platform should be the Rumble variant of the enum


  Scenario: Happy path Nico Video URL was given
    Given I have a proper Nico Video URL
    When I parse it via parse_platform
    Then the resulting platform should be the Nico variant of the enum

