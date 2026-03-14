# Git Submodule이란?

다른 Git 저장소를 내 저장소 안에 **특정 커밋에 고정해서** 포함시키는 기능이다.

## 왜 쓰는가

외부 라이브러리 소스코드가 필요할 때, 직접 복사해서 넣으면:

- 원본 저장소와의 연결이 끊긴다
- 업데이트 추적이 안 된다
- 코드가 중복된다

Submodule을 쓰면 "이 저장소의 이 커밋을 여기에 넣어라"는 **참조(포인터)** 만 저장한다.

## 실제로 저장되는 것

- `.gitmodules` — submodule URL과 경로 매핑
- Git 내부에 해당 submodule이 가리키는 **커밋 해시**

전체 소스를 내 저장소에 복사하는 게 아니라, 어떤 커밋을 참조하는지만 기록하는 것이다.

## 사용법

### 추가

```bash
git submodule add https://github.com/example/lib.git path/to/lib
```

### Clone

```bash
# clone할 때 submodule까지 한번에
git clone --recurse-submodules <repo-url>

# 이미 clone한 후 submodule을 가져오기
git submodule update --init --recursive
```

### 업데이트

submodule을 최신 커밋으로 올리고 싶을 때:

```bash
cd path/to/lib
git pull origin main
cd ..
git add path/to/lib
git commit -m "update submodule"
```

## 주의할 점

- 그냥 `git clone`하면 submodule 디렉토리가 **비어있다**. 반드시 `--recurse-submodules` 옵션을 쓰거나, clone 후 `git submodule update --init --recursive`를 실행해야 한다.
- submodule은 특정 커밋에 고정되어 있으므로, 원본 저장소가 업데이트되어도 자동으로 따라가지 않는다. 명시적으로 업데이트해야 한다.
