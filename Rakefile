require 'os'
require 'fileutils'
require 'json'

Gemstone = Struct.new(:gem, :qty)

target = "./rrake"
tp = "target/release/rrake"
add = "test/add.rb"

if OS.windows? then
    target = "rrake.exe"
    tp = "target\\release\\rrake.exe"
    add = "test\\add.rb"
end

task :default do
    sh "cargo build --release"
end

task :test do
    sh "#{target} --help"
    puts
    sh "#{target} --version"
    puts
    sh "#{target} -f Rakefile echo"
    puts
    sh "#{target} -f Rakefile refertoecho"
    puts
    sh "#{target} -f Rakefile pwd"
    puts
    sh "#{target} --file Rakefile gemstone"
    puts
    sh "#{target} --rakefile Rakefile countdown"
    puts
    sh "#{target} --rakefile Rakefile ruby"
end

task :upx => [:default] do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "upx -9 #{tp} -o #{target}"
end

task :clean do
    sh "cargo clean"
end

task :cleanlock do
    File.delete("Cargo.lock")
end

task :echo do
    puts "Simple echo task..."
    puts "Prints out a string!"
end

task :sh do
    sh "touch dummy.txt"
end

task :cleansh do
    File.delete("dummy.txt")
end

task :pwd do
    Dir.pwd
end

task :gemstone do
    gemstone = Gemstone.new("ruby", 3)
    puts gemstone.to_h.to_json
end

task :countdown do
    # TODO implement loops
    puts "5"
    sleep 1000
    puts "4"
    sleep 1000
    puts "3"
    sleep 1000
    puts "2"
    sleep 1000
    puts "1"
    sleep 1000
    puts "Blast off!"
end

task :ruby do
    ruby "#{add}"
end

task :refertoecho => [:echo]
