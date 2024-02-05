Feature: Parsing stuff with parse_platform

  Scenario:: Happy path youtube url was given
    Given I have a proper youtube.com video URL
    When I parse it via parse_platform
    Then the resulting platform should be the Youtube variant of the enum

  Scenario: Happy path youtube url was given
    Given I have a proper youtu.be video URL
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

  Scenario: Happy path BitChute video URL was given
    Given I have a proper Bitchute video URL
    When I parse it via parse_platform
    Then the resulting platform should be the BitChute variant of the enum

  Scenario: Video URL from an unknown platform was given
    Given I have a proper Fartstream video URL
    When I parse it via parse_platform
    Then the resulting platform should be the Unknown variant of the enum


  Scenario: Youtube with path arg id was given
    Given I have a youtube url where the asset id is part of the url segment
    When I parse it with asset_id
    Then I should have the correct video id as string

  Scenario: Youtube with query arg id was given
    Given I have a youtube url where the asset id is part of the query
    When I parse it with asset_id
    Then I should have the correct video id as string

  Scenario: No segment id, query is present, but not an asset id
    Given I have a youtube URL where there is a query but no asset_id
    When I parse it with asset_id
    Then I should have a None result for the asset_id

  Scenario: Parse a video id for Rumble
    Given I have a proper rumble video URL
    When I parse it with asset_id
    Then I should have the correct video id as string

