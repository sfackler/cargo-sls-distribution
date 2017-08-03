#!/usr/bin/env ruby

require 'octokit'

target = ARGV[0]
name = ARGV[1]

username = ENV["CIRCLE_PROJECT_USERNAME"]
password = ENV["GITHUB_API_PASSWORD"]

client = Octokit::Client.new(:login => username, :password => password)

reponame = ENV["CIRCLE_PROJECT_REPONAME"]
project = "#{username}/#{reponame}"
tag = ENV["CIRCLE_TAG"]
release_url = client.release_for_tag(project, tag).url

path = "target/#{target}/release/#{name}"
options = {
    :content_type => "application/octet-stream",
    :name => "#{name}-#{target}",
}

asset = client.upload_asset(release_url, path, options)

puts asset.url
