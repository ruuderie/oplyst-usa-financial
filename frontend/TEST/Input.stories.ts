import type { Meta, StoryObj } from '@storybook/vue3';

import Input from '../components/ui/input/Input.vue';

const meta = {
  title: 'Input',
  component: Input,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Input>;

export default meta;
type Story = StoryObj<typeof Input>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};