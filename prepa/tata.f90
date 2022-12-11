
! gfortran tata.f90 -o tata && ./tata

function is_valid(c) result(res)
    character :: c
    integer :: res
    res = VERIFY(c, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
end function is_valid


function get_priority(c) result(res)
    character :: c
    integer :: res
    if( c >= 'a' .and. c <= 'z') then 
        res = 1 + iachar(c) - iachar('a')
    else if( c>= 'A' .and. c <= 'Z') then
        res = 27 + iachar(c) - iachar('A')
    end if
end function get_priority

function get_length(str) result(size)
    implicit none
    integer :: size, i
    character(len=200) :: str
    integer :: is_valid
    
    do i = 1, 200
        if (is_valid(str(i:i)) /= 0) then
            exit
        end if
    end do
    size = i-1
end function get_length

function is_in(str, c) result(res)

    character(len=52) :: str
    character :: c
    integer :: res, i
    res = 0
    do i = 1,52
        if(str(i:i) == c) then
            res = 1
            exit
        end if
    end do

end function is_in;


function is_in_v2(str, c) result(res)

    character(len=200) :: str
    character :: c
    integer :: res, i, length, get_length
    res = 0
    length = get_length(str)
    do i = 1,length
        if(str(i:i) == c) then
            res = 1
            exit
        end if
    end do

end function is_in_v2;

function reset_52(str) result(res)

    character(len=52) :: str
    integer :: i, res
    do i = 1,52
        str(i:i) = "_"
    end do
    res = 1
end function reset_52;

program tata
    
    ! Local variables
    integer, parameter :: read_unit = 99
    integer :: ios
    character(len=200), allocatable :: inputs(:)
    character(len=200) :: line
    character(len=52) :: found_chars
    integer :: found_chars_ptr
    integer :: nb_lines, i,j,k,l, cond_1, cond_2, cond_3
    character :: c
    integer :: answ1, answ2
    integer :: get_length, get_priority, reset_52

    ! Read input from file (did not manage to put this in a function...)
    open(unit=read_unit, file="input.txt", iostat=ios)
    if ( ios /= 0 ) stop "Error opening file"
    nb_lines = 0
    do
        read(read_unit, '(A)', iostat=ios) line
        if (ios /= 0) exit
        nb_lines = nb_lines + 1
    end do
    allocate(inputs(nb_lines))
    rewind(read_unit)
    do i = 1, nb_lines
        read(read_unit, '(A)') inputs(i)
    end do
    close(read_unit)

    !do i = 1, nb_lines
    !    print*, inputs(i)
    !end do
    
    ! Part 1
    answ1 = 0
    do i = 1, nb_lines
        found_chars_ptr = reset_52(found_chars)
        line = inputs(i)
        l = get_length(inputs(i))
        do j = 1, (l/2)
            do k = l/2+1, l
                if (line(j:j) == line(k:k) .and. (is_in(found_chars, line(j:j)) == 0)) then
                    found_chars(found_chars_ptr:found_chars_ptr) = line(j:j)
                    found_chars_ptr = found_chars_ptr + 1
                    answ1 = answ1 + get_priority(line(j:j))
                end if
            end do
        end do
    end do
    print*, "The first answer is", answ1
    
    ! Part 2
    answ2 = 0
    do i = 1, nb_lines/3
        found_chars_ptr = reset_52(found_chars)
        line = inputs((i-1)*3+1)
        l = get_length(line)
        do j = 1, l
            c = line(j:j)
            cond_1 = is_in(found_chars, c)
            cond_2 = is_in_v2(inputs((i-1)*3+2), c)
            cond_3 = is_in_v2(inputs((i-1)*3+3), c)
            if ( (cond_1 == 0) .and. (cond_2 == 1) .and. (cond_3 == 1) )then
                found_chars(found_chars_ptr:found_chars_ptr) = c
                found_chars_ptr = found_chars_ptr + 1
                answ2 = answ2 + get_priority(c)
            end if
        end do
    end do
    print*, "The second answer is", answ2    
    

end program tata
