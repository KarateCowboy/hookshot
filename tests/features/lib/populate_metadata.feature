Feature: Populating a new videos metadata from its platform source

  Scenario: Youtube metadata happy path
    Given I have a youtube Video
    When I fetch the metadata for it
    Then it should have the video upload information