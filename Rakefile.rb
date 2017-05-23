require 'os'

#target = "rrake"
#tp = "target/release/#{target}"

if OS.windows? then
    target = "rrake.exe"
    tp = "target\\release\\rrake.exe"
end

task :default do
    sh "cargo build --release"
end

task :test do
    sh "#{tp} --help"
    puts
    sh "#{tp} --version"
end

task :upx => [:default] do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "upx -9 #{tp} -o #{target}"
end

task :clean => [:cleanupx] do
    sh "cargo clean"
end

task :cleanupx do
    if File.exists?(target) then
        File.delete(target)
    end
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
