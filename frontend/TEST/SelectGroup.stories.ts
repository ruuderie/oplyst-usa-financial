import type { Meta, StoryObj } from '@storybook/vue3';

import SelectGroup from '../components/ui/select/SelectGroup.vue';

const meta = {
  title: 'SelectGroup',
  component: SelectGroup,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SelectGroup>;

export default meta;
type Story = StoryObj<typeof SelectGroup>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};